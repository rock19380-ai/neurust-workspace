"use client";

import { useState, useRef, useEffect } from "react";
import { 
  CreditCard, 
  Terminal, 
  LogOut, 
  History, 
  Wallet,
  Zap 
} from "lucide-react";
import { useWallet } from "@solana/wallet-adapter-react";
import Link from "next/link";

interface WalletNavProps {
  credits: number | string;
}

export default function WalletNav({ credits }: WalletNavProps) {
  const { publicKey, disconnect } = useWallet();
  const [isOpen, setIsOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Click outside to close menu
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  if (!publicKey) return null;

  // Format Address (e.g., 7XwP...9zJQ)
  const shortAddress = `${publicKey.toBase58().slice(0, 4)}...${publicKey.toBase58().slice(-4)}`;
  const isLowCredit = typeof credits === 'number' && credits < 200;

  return (
    <div className="relative" ref={menuRef}>
      {/* Trigger Button */}
      <button 
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-3 pl-4 pr-1 py-1 rounded-full bg-[#111] border border-white/10 hover:border-[#FF7E5F]/50 transition-all group"
      >
        {/* Credits */}
        <div className={`flex items-center gap-1.5 text-xs font-mono font-medium ${isLowCredit ? 'text-red-400' : 'text-emerald-400'}`}>
          <Zap size={14} className={isLowCredit ? "animate-pulse" : ""} />
          {credits}
        </div>

        {/* Separator */}
        <div className="h-4 w-[1px] bg-white/10"></div>

        {/* Avatar */}
        <div className="h-8 w-8 rounded-full bg-gradient-to-tr from-[#FF7E5F] to-purple-600 p-[1px]">
          <div className="h-full w-full rounded-full bg-black flex items-center justify-center">
            <Wallet size={14} className="text-white group-hover:text-[#FF7E5F] transition-colors" />
          </div>
        </div>
      </button>

      {/* Dropdown Menu */}
      {isOpen && (
        <div className="absolute right-0 mt-3 w-64 origin-top-right rounded-xl bg-[#0a0a0a] border border-white/10 shadow-2xl z-50 overflow-hidden animate-in fade-in zoom-in-95 duration-200">
          
          {/* Header */}
          <div className="px-5 py-4 bg-white/5 border-b border-white/10">
            <p className="text-[10px] uppercase tracking-widest text-slate-500 mb-1">Connected Wallet</p>
            <div className="flex items-center justify-between">
              <span className="font-mono text-white text-sm">{shortAddress}</span>
              <span className="h-2 w-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]"></span>
            </div>
          </div>

          {/* Menu Items */}
          <div className="py-2">
            <Link href="/cli-access" onClick={() => setIsOpen(false)} className="group flex items-center px-5 py-2.5 text-sm text-slate-300 hover:bg-white/5 hover:text-white transition-colors">
              <Terminal className="mr-3 h-4 w-4 text-slate-500 group-hover:text-[#FF7E5F]" />
              CLI Access Key
            </Link>
            
            <Link href="/billing" onClick={() => setIsOpen(false)} className="group flex items-center px-5 py-2.5 text-sm text-slate-300 hover:bg-white/5 hover:text-white transition-colors">
              <CreditCard className="mr-3 h-4 w-4 text-slate-500 group-hover:text-[#FF7E5F]" />
              Billing & Top Up
            </Link>

            <Link href="/activity" onClick={() => setIsOpen(false)} className="group flex items-center px-5 py-2.5 text-sm text-slate-300 hover:bg-white/5 hover:text-white transition-colors">
              <History className="mr-3 h-4 w-4 text-slate-500 group-hover:text-[#FF7E5F]" />
              History
            </Link>
          </div>

          {/* Disconnect */}
          <div className="py-2 border-t border-white/10 bg-white/[0.02]">
            <button 
              onClick={() => {
                disconnect();
                setIsOpen(false);
              }}
              className="group flex w-full items-center px-5 py-2 text-sm text-red-400 hover:text-red-300 hover:bg-red-500/10 transition-colors"
            >
              <LogOut className="mr-3 h-4 w-4" />
              Disconnect
            </button>
          </div>
        </div>
      )}
    </div>
  );
}