"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { useParams, useRouter } from "next/navigation";
import { ArrowLeft, BookOpenText, ShieldCheck, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useWallet } from "@solana/wallet-adapter-react"; // 🔥 Added Hook

interface Project {
  id: string;
  name: string;
  description: string;
  framework: "Anchor" | "Native";
  createdAt: string;
}

type TabKey = "overview" | "audit" | "settings";

const TAB_ITEMS: { key: TabKey; label: string; icon: any }[] = [
  { key: "overview", label: "Overview", icon: BookOpenText },
  { key: "audit", label: "Audit", icon: ShieldCheck },
  { key: "settings", label: "Settings", icon: Trash2 },
];

const defaultCode = `use anchor_lang::prelude::*;
// ... (Your default code block) ...
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
}
// ...
`;

export default function ProjectDetailsPage() {
  const { publicKey } = useWallet(); // 🔥 Get Wallet
  const params = useParams<{ id: string }>();
  const router = useRouter();
  const projectId = params?.id;
  const [project, setProject] = useState<Project | null>(null);
  // ... (Other state variables remain same) ...
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<TabKey>("overview");

  const [code, setCode] = useState<string>(defaultCode);
  const [auditLoading, setAuditLoading] = useState(false);
  const [auditError, setAuditError] = useState<string | null>(null);
  const [auditResult, setAuditResult] = useState<string>("");
  const [deleteLoading, setDeleteLoading] = useState(false);

  // ... (fetchProject useEffect remains same) ...
  useEffect(() => {
    const fetchProject = async () => {
      if (!projectId) return;
      setLoading(true);
      setError(null);
      try {
        const res = await fetch(`/api/projects/${projectId}`, { cache: "no-store" });
        if (!res.ok) {
            // ...
             const body = await res.json().catch(() => null);
             throw new Error(body?.message || "Failed to load project");
        }
        const data = await res.json();
        setProject(data?.project ?? null);
      } catch (err: any) {
        setError(err?.message || "Failed to load project");
      } finally {
        setLoading(false);
      }
    };
    fetchProject();
  }, [projectId]);

  // ... (stats useMemo remains same) ...
  const stats = useMemo(() => {
    return [
      { title: "Status", value: "Active", hint: "All systems green" },
      { title: "Files", value: "3 Files", hint: "Workspace synced" },
      { title: "Created", value: project ? new Date(project.createdAt).toLocaleDateString() : "-", hint: "Project inception" },
      { title: "Last Audit", value: "Just now", hint: "AI Smart Audit" },
    ];
  }, [project]);

  const handleRunAudit = async () => {
    if (!publicKey) {
        setAuditError("Please connect wallet to audit.");
        return;
    }
    
    setAuditLoading(true);
    setAuditError(null);
    setAuditResult("");
    try {
      // 🔥 Updated Fetch with Wallet Header
      const res = await fetch("/api/agent/audit", {
        method: "POST",
        headers: { 
            "Content-Type": "application/json",
            "x-neurust-wallet": publicKey.toBase58() // 🔥 Critical for Backend Auth
        },
        body: JSON.stringify({ code }),
      });
      if (!res.ok) {
        const body = await res.json().catch(() => null);
        throw new Error(body?.message || "Audit failed");
      }
      const data = await res.json();
      setAuditResult(data?.report || JSON.stringify(data, null, 2));
    } catch (err: any) {
      setAuditError(err?.message || "Audit failed");
    } finally {
      setAuditLoading(false);
    }
  };

  // ... (handleDelete remains same, maybe add header there too if needed) ...
  const handleDelete = async () => {
      // ...
    if (!projectId) return;
    const confirmed = typeof window !== "undefined" ? window.confirm("Are you sure you want to delete this project?") : false;
    if (!confirmed) return;

    setDeleteLoading(true);
    setError(null);
    try {
      const res = await fetch(`/api/projects/${projectId}`, { method: "DELETE" });
      if (!res.ok) {
          const body = await res.json().catch(() => null);
          throw new Error(body?.message || "Failed to delete project");
      }
      router.push("/dashboard");
    } catch (err: any) {
      setError(err?.message || "Failed to delete project");
    } finally {
      setDeleteLoading(false);
    }
  };

  // ... (renderTab and return JSX remains EXACTLY the same) ...
  // (UI Code က ခင်ဗျားမူရင်းအတိုင်းမို့ ပြန်မရေးတော့ပါဘူး၊ Header Logic ပဲဖြည့်တာပါ)
  
  const renderTab = () => {
    // ... (Your exact existing renderTab logic) ...
    if (activeTab === "overview") {
        return (
          <div className="space-y-6">
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
              {stats.map((stat) => (
                <div
                  key={stat.title}
                  className="rounded-2xl border border-white/10 bg-white/5 backdrop-blur p-4 shadow-[0_0_20px_rgba(255,126,95,0.15)]"
                >
                  <p className="text-sm text-slate-400">{stat.title}</p>
                  <p className="mt-2 text-2xl font-bold text-white">{stat.value}</p>
                  <p className="text-xs text-emerald-400">{stat.hint}</p>
                </div>
              ))}
            </div>
  
            <div className="rounded-2xl border border-white/10 bg-gradient-to-br from-white/5 via-black/30 to-white/5 p-6 shadow-[0_0_32px_rgba(255,126,95,0.12)]">
              <p className="text-xs uppercase tracking-[0.25em] text-slate-400">Project Readme</p>
              <h3 className="mt-2 text-2xl font-semibold text-white">{project?.name || "Loading..."}</h3>
              <p className="mt-4 whitespace-pre-line text-sm text-slate-300 leading-7">
                {`# Welcome to ${project?.name || "your project"}\n\nThis is your Electric Rust workspace. Drop your Anchor or Native program code here, run Smart Audits, and track findings.\n\n## Quickstart\n- Use the Audit tab to paste your program code.\n- Click "Run Smart Audit" to get instant AI security review.\n- Manage permissions & secrets in Settings (coming soon).\n\nHappy building!`}
              </p>
            </div>
          </div>
        );
      }
  
      if (activeTab === "audit") {
        return (
          <div className="space-y-4">
            <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <p className="text-xs uppercase tracking-[0.25em] text-slate-400">AI Security Auditor</p>
                <h3 className="text-xl font-semibold text-white">Paste your Rust / Anchor code</h3>
              </div>
              <Button
                variant="default" // Changed to default
                onClick={handleRunAudit}
                disabled={auditLoading || !code.trim()}
                className="min-w-[170px] justify-center bg-[#FF7E5F] text-black hover:bg-[#FF7E5F]/90"
              >
                {auditLoading ? "Auditing..." : "Run Smart Audit"}
              </Button>
            </div>
  
            <textarea
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className={cn(
                "w-full min-h-[260px] rounded-2xl border border-white/10 bg-[#0d1117]",
                "font-mono text-sm text-slate-100 p-4 shadow-[0_0_30px_rgba(0,0,0,0.35)]",
                "focus:outline-none focus:ring-2 focus:ring-[#FF7E5F]/60"
              )}
              placeholder="// Paste your program here"
            />
  
            {auditError && (
              <div className="rounded-lg border border-rose-500/40 bg-rose-500/10 px-3 py-2 text-sm text-rose-100">
                {auditError}
              </div>
            )}
  
            {auditResult && (
              <div className="rounded-2xl border border-white/10 bg-gradient-to-br from-white/5 via-black/40 to-white/5 p-5 shadow-[0_0_30px_rgba(255,126,95,0.18)]">
                <p className="text-xs uppercase tracking-[0.2em] text-slate-400">AI Report</p>
                <div className="mt-3 whitespace-pre-wrap text-sm leading-7 text-slate-100">
                  {auditResult}
                </div>
              </div>
            )}
          </div>
        );
      }
  
      if (activeTab === "settings") {
        return (
          <div className="space-y-4">
            <div className="rounded-2xl border border-white/10 bg-white/5 p-6 shadow-[0_0_24px_rgba(255,126,95,0.12)]">
              <p className="text-xs uppercase tracking-[0.25em] text-slate-400">Danger Zone</p>
              <h3 className="mt-2 text-xl font-semibold text-white">Delete project</h3>
              <p className="mt-2 text-sm text-slate-300">
                This will remove the project from your local workspace. (Remote deletion coming soon.)
              </p>
              <Button
                variant="outline"
                className="mt-4 border-rose-500/60 text-rose-300 hover:bg-rose-500/10 hover:text-rose-100"
                onClick={handleDelete}
                disabled={deleteLoading}
              >
                <Trash2 className="mr-2 h-4 w-4" /> {deleteLoading ? "Deleting..." : "Delete Project"}
              </Button>
            </div>
          </div>
        );
      }
  
      return null;
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3 text-sm text-slate-400">
        <Link href="/dashboard" className="flex items-center gap-2 text-[#FF7E5F] hover:text-[#ff9b83]">
          <ArrowLeft className="h-4 w-4" /> Back to Dashboard
        </Link>
        <span className="text-slate-600">/</span>
        <span>{project?.name || "Loading..."}</span>
      </div>

      <div className="rounded-2xl border border-white/10 bg-gradient-to-r from-white/5 via-[#0b0b0b]/70 to-white/5 p-6 shadow-[0_0_32px_rgba(255,126,95,0.14)]">
        {loading ? (
          <div className="text-slate-300">Loading project...</div>
        ) : error ? (
          <div className="text-rose-300">{error}</div>
        ) : !project ? (
          <div className="text-slate-300">Project not found.</div>
        ) : (
          <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
            <div>
              <p className="text-xs uppercase tracking-[0.25em] text-slate-400">Project</p>
              <h1 className="text-3xl font-bold text-white">{project.name}</h1>
              <p className="mt-2 text-sm text-slate-300">{project.description}</p>
            </div>
            <div className="flex items-center gap-3">
              <span className="rounded-full border border-[#FF7E5F]/50 bg-[#FF7E5F]/10 px-4 py-2 text-sm font-semibold text-[#FF7E5F] shadow-[0_0_16px_rgba(255,126,95,0.4)]">
                {project.framework}
              </span>
              <span className="rounded-full border border-cyan-400/50 bg-cyan-400/10 px-4 py-2 text-sm text-cyan-200">
                Created {new Date(project.createdAt).toLocaleDateString()}
              </span>
            </div>
          </div>
        )}
      </div>

      <div className="rounded-2xl border border-white/10 bg-white/5 p-2 shadow-[0_0_18px_rgba(255,126,95,0.08)]">
        <div className="flex flex-wrap gap-2 p-2">
          {TAB_ITEMS.map((tab) => {
            const Icon = tab.icon;
            const active = activeTab === tab.key;
            return (
              <button
                key={tab.key}
                onClick={() => setActiveTab(tab.key)}
                className={cn(
                  "flex items-center gap-2 rounded-xl px-4 py-2 text-sm transition",
                  active
                    ? "bg-[#FF7E5F]/15 text-white border border-[#FF7E5F]/60 shadow-[0_0_18px_rgba(255,126,95,0.5)]"
                    : "bg-transparent text-slate-300 hover:bg-white/5 border border-transparent"
                )}
              >
                <Icon className="h-4 w-4" />
                {tab.label}
              </button>
            );
          })}
        </div>
      </div>

      <div className="rounded-2xl border border-white/10 bg-white/5 p-6 shadow-[0_0_24px_rgba(0,0,0,0.45)]">
        {renderTab()}
      </div>
    </div>
  );
}