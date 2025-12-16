"use client";

import Link from "next/link";
import { LogIn } from "lucide-react";
// 👇 Path အမှန် (app folder ကို ထည့်ပေးရပါမယ်)
import { useAuth } from "@/app/context/auth-context"; 
import WalletNav from "./wallet-nav"; 

export const Navbar = () => {
  const { user } = useAuth();

  return (
    <header className="sticky top-0 z-50 w-full bg-[#050505]/80 backdrop-blur-xl border-b border-white/5">
      <div className="mx-auto flex w-full max-w-7xl items-center justify-between px-6 py-4 text-white">
        
        {/* Left: Logo Section */}
        <Link
          href="/"
          className="flex items-center gap-3 text-lg font-bold tracking-widest group"
        >
          <span className="flex items-center text-2xl font-mono">
            <span className="text-slate-500">&gt;</span>
            <span className="text-[#FF7E5F] drop-shadow-[0_0_10px_rgba(255,126,95,0.8)] group-hover:animate-pulse">_</span>
          </span>
          <div className="flex flex-col leading-none">
            <span className="text-white tracking-tighter">NEURUST</span>
            <span className="text-[9px] text-slate-500 tracking-[0.2em] group-hover:text-[#FF7E5F] transition-colors">AI CLI</span>
          </div>
        </Link>

        {/* Right: Auth Action Section */}
        <div className="flex items-center gap-6">
            
            {user ? (
                <WalletNav credits={user.credits} />
            ) : (
                <Link
                  href="/login"
                  className="flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-5 py-2 text-xs font-semibold uppercase tracking-widest text-slate-300 transition hover:border-[#FF7E5F]/50 hover:text-white hover:bg-[#FF7E5F]/10"
                >
                  <LogIn size={14} />
                  Login / Authorize
                </Link>
            )}

        </div>
      </div>
    </header>
  );
};