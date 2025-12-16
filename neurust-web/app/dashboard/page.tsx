"use client";

import Link from "next/link";
import { useEffect, useMemo, useState } from "react";
import CreateProjectModal from "@/components/dashboard/create-project-modal";
import { useWallet } from "@solana/wallet-adapter-react"; 

interface Project {
  id: string;
  name: string;
  description: string;
  framework: string;
  createdAt: string;
}

export default function DashboardPage() {
  const { publicKey } = useWallet(); 
  const [projects, setProjects] = useState<Project[]>([]);
  const [credits, setCredits] = useState<number>(0); 
  const [userRole, setUserRole] = useState<string>("User");
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!publicKey) return;

    const fetchData = async () => {
      setLoading(true);
      try {
        const walletHeader = { "x-neurust-wallet": publicKey.toBase58() };

        // 1. Fetch User Profile
        const userRes = await fetch("/api/user", { headers: walletHeader });
        if (userRes.ok) {
            const userData = await userRes.json();
            setCredits(userData.credits || 0);
            setUserRole(userData.role || "User");
        }

        // 2. Fetch Projects (Metadata only)
        const projRes = await fetch("/api/projects", { 
            headers: walletHeader,
            cache: "no-store" 
        });
        
        if (projRes.ok) {
             const projData = await projRes.json();
             // Ensure we extract the array correctly
             const list = Array.isArray(projData) ? projData : (projData.projects || []);
             setProjects(list);
        }
        
      } catch (err: any) {
        // Silent fail allows dashboard to load even if backend isn't perfect yet
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [publicKey]);

  // 🔥 Credit Display Logic
  const displayCredits = useMemo(() => {
      if (loading) return "...";
      // Admin or Rich User -> Show Infinity
      if (userRole === "SuperAdmin" || userRole === "Admin" || credits > 900000) {
          return "∞";
      }
      return credits;
  }, [credits, userRole, loading]);

  const stats = useMemo(() => {
    return [
      { 
        title: "Total Projects", 
        value: projects.length, 
        change: projects.length ? "+ active" : "Start building" 
      },
      { 
        title: "Audits Run", 
        value: "N/A", 
        change: "Lifetime" 
      },
      { 
        title: "Credits Remaining", 
        value: displayCredits, 
        change: displayCredits === "∞" ? "Unlimited Access" : (credits < 10 ? "Top up needed" : "Ready to build") 
      },
    ];
  }, [projects.length, credits, displayCredits]);

  // Just strictly for display
  const formatDate = (date: string) => {
    try { return new Date(date).toLocaleDateString(); } catch { return "-"; }
  };

  if (!publicKey) {
      return <div className="p-10 text-center text-slate-500">Please connect your wallet.</div>;
  }

  return (
    <div className="space-y-8">
      {/* Stats Section */}
      <section className="grid gap-4 md:grid-cols-3">
        {stats.map((stat) => (
          <div key={stat.title} className="rounded-2xl border border-white/10 bg-white/5 p-5">
            <p className="text-sm text-slate-400">{stat.title}</p>
            <p className="mt-2 text-3xl font-bold text-white">{stat.value}</p>
            <p className="mt-1 text-xs text-emerald-400">{stat.change}</p>
          </div>
        ))}
      </section>

      <section className="grid gap-6 lg:grid-cols-3">
        {/* Project List */}
        <div className="lg:col-span-2 rounded-2xl border border-white/10 bg-white/5 p-6">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-semibold">Your Projects</h2>
            {/* Modal won't create files anymore, just shows CLI command */}
            <CreateProjectModal /> 
          </div>

          <div className="space-y-4">
            {projects.length === 0 && (
              <div className="p-6 text-center text-slate-500 border border-dashed border-white/10 rounded-xl">
                No projects found. Run <code className="text-[#FF7E5F]">neurust new</code> in your terminal.
              </div>
            )}
            {projects.map((p) => (
              <div key={p.id} className="p-4 rounded-xl bg-white/5 border border-white/5 flex justify-between items-center">
                <div>
                    <h3 className="font-bold text-white">{p.name}</h3>
                    <p className="text-xs text-slate-400">{p.framework} • {formatDate(p.createdAt)}</p>
                </div>
                <div className="px-3 py-1 rounded-full bg-[#FF7E5F]/10 text-[#FF7E5F] text-xs border border-[#FF7E5F]/20">
                    Active
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* CTA */}
        <div className="rounded-2xl border border-white/10 bg-gradient-to-br from-white/10 to-transparent p-6">
          <h3 className="text-xl font-bold mb-2">New Project?</h3>
          <p className="text-sm text-slate-300 mb-4">
            Use the CLI to scaffold a new project with AI assistance.
          </p>
          <div className="bg-black/50 p-3 rounded-lg font-mono text-sm text-[#FF7E5F]">
            $ neurust new my_app
          </div>
        </div>
      </section>
    </div>
  );
}