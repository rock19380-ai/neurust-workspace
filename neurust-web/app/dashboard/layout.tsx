"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { ReactNode } from "react";
import { ClipboardList, FolderKanban, Settings } from "lucide-react";
import { useWallet } from "@solana/wallet-adapter-react";

const navItems = [
  { name: "Projects", href: "/dashboard", icon: FolderKanban },
  { name: "Audits", href: "/dashboard/audits", icon: ClipboardList },
  { name: "Settings", href: "/dashboard/settings", icon: Settings },
];

const neonRust = "#FF7E5F";

function truncateAddress(address: string) {
  if (!address) return "-";
  return `${address.slice(0, 4)}...${address.slice(-4)}`;
}

export default function DashboardLayout({ children }: { children: ReactNode }) {
  const { publicKey } = useWallet(); // Get real wallet
  const walletAddress = publicKey ? publicKey.toBase58() : "Not Connected";
  const pathname = usePathname();
  const credits = 420;

  return (
    <div className="min-h-screen bg-gradient-to-br from-black via-[#0a0a0a] to-[#050505] text-white flex">
      <aside className="w-64 border-r border-white/5 bg-white/5 backdrop-blur-lg p-6 flex flex-col gap-6">
        <div className="flex items-center gap-3">
          <span className="text-sm uppercase tracking-[0.3em] text-slate-400">Dashboard</span>
        </div>
        <nav className="space-y-2">
          {navItems.map((item) => {
            const Icon = item.icon;
            const active = pathname === item.href || pathname.startsWith(item.href + "/");
            return (
              <Link
                key={item.name}
                href={item.href}
                className={`group flex items-center gap-3 rounded-xl px-3 py-2 transition-all border border-transparent hover:border-white/10 hover:bg-white/5 ${
                  active
                    ? "bg-white/10 border-white/10 shadow-[0_0_12px_rgba(255,126,95,0.45)]"
                    : ""
                }`}
              >
                <div
                  className={`p-2 rounded-lg transition-all ${
                    active ? "text-black" : "text-slate-300"
                  }`}
                  style={active ? { background: neonRust, boxShadow: "0 0 12px rgba(255,126,95,0.6)" } : {}}
                >
                  <Icon className="h-5 w-5" />
                </div>
                <span className="font-medium text-sm">{item.name}</span>
              </Link>
            );
          })}
        </nav>
        <div className="mt-auto rounded-2xl border border-white/10 bg-white/5 backdrop-blur p-4">
          <p className="text-xs uppercase tracking-[0.2em] text-slate-400 mb-2">Wallet</p>
          <p className="text-lg font-semibold">{truncateAddress(walletAddress)}</p>
          <p className="mt-4 text-xs uppercase tracking-[0.2em] text-slate-400">Credits</p>
          <p className="text-xl font-bold" style={{ color: neonRust, textShadow: "0 0 12px rgba(255,126,95,0.7)" }}>
            {credits}
          </p>
        </div>
      </aside>
      <main className="flex-1 p-8 space-y-6">
        <header className="flex items-center justify-between rounded-2xl border border-white/10 bg-white/5 backdrop-blur px-6 py-4">
          <div>
            <p className="text-sm text-slate-400">Signed in as</p>
            <p className="text-xl font-semibold">{truncateAddress(walletAddress)}</p>
          </div>
          <div className="text-right">
            <p className="text-sm text-slate-400">Credits</p>
            <p className="text-2xl font-bold" style={{ color: neonRust, textShadow: "0 0 14px rgba(255,126,95,0.8)" }}>
              {credits}
            </p>
          </div>
        </header>
        {children}
      </main>
    </div>
  );
}
