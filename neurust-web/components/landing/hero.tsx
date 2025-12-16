"use client";

import Link from "next/link";
import { motion } from "framer-motion";
import { useEffect, useState } from "react";

// The sequence of the terminal interaction
const SEQUENCE = [
  { text: "$ neurust create my-solana-dapp", type: "input", delay: 0 },
  { text: "🤖 Neurust: Analyzing request...", type: "output", delay: 1500 },
  { text: "> Select framework: [Next.js] / React / Vue", type: "output", highlight: true, delay: 2500 },
  { text: "🏗️ Scaffolding project in ./my-solana-dapp...", type: "output", delay: 3500 },
  { text: "✅ Done! Run 'cd my-solana-dapp && npm run dev'", type: "success", delay: 5000 },
];

export const Hero = () => {
  const [step, setStep] = useState(0);
  const [typedInput, setTypedInput] = useState("");

  useEffect(() => {
    // 1. Typing animation for the first command
    const command = SEQUENCE[0].text;
    let charIndex = 0;
    
    const typeInterval = setInterval(() => {
      setTypedInput(command.slice(0, charIndex + 1));
      charIndex++;
      if (charIndex === command.length) {
        clearInterval(typeInterval);
        // Start showing subsequent lines
        startSequence();
      }
    }, 50);

    return () => clearInterval(typeInterval);
  }, []);

  const startSequence = () => {
    SEQUENCE.slice(1).forEach((line, index) => {
      setTimeout(() => {
        setStep((prev) => prev + 1);
      }, line.delay - 1000); // Adjust timing relative to typing finish
    });
  };

  return (
    <section className="mx-auto flex max-w-6xl flex-col gap-16 px-6 py-24 text-white lg:flex-row lg:items-center">
      
      {/* Left Content */}
      <div className="flex-1 space-y-8">
        <p className="inline-block rounded-full bg-[#FF7E5F]/10 px-3 py-1 text-xs font-bold uppercase tracking-[0.2em] text-[#FF7E5F]">
          Electric Rust Engine
        </p>
        <h1 className="text-5xl font-bold leading-tight md:text-7xl">
          The <span className="text-transparent bg-clip-text bg-gradient-to-r from-[#FF7E5F] to-[#00F2EA]">AI Co-Pilot</span> for Solana Devs
        </h1>
        <p className="max-w-xl text-lg text-slate-400 leading-relaxed">
          Build, audit, and deploy secure Anchor programs with an autonomous AI agent that understands your entire toolchain.
        </p>
        <div className="flex flex-wrap gap-4 pt-4">
          <Link
            href="/get-started"
            className="group relative inline-flex items-center justify-center overflow-hidden rounded-full bg-[#FF7E5F] px-8 py-4 font-bold text-black transition-transform active:scale-95"
          >
            <span className="absolute inset-0 bg-white/20 group-hover:translate-x-full transition-transform duration-500 ease-out -translate-x-full skew-x-12"></span>
            <span className="relative">GET STARTED</span>
          </Link>
          <Link
            href="/docs"
            className="inline-flex items-center justify-center rounded-full border border-slate-700 px-8 py-4 font-bold text-slate-300 transition hover:bg-white/5 hover:text-white"
          >
            DOCUMENTATION
          </Link>
        </div>
      </div>

      {/* Right Content: Advanced Terminal Animation */}
      <div className="flex-1 w-full max-w-lg">
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5 }}
          className="overflow-hidden rounded-xl border border-white/10 bg-[#0A0A0A] shadow-2xl shadow-[#FF7E5F]/10"
        >
          {/* Terminal Header */}
          <div className="flex items-center gap-2 border-b border-white/5 bg-[#111] px-4 py-3">
            <div className="flex gap-2">
              <span className="h-3 w-3 rounded-full bg-red-500/50" />
              <span className="h-3 w-3 rounded-full bg-yellow-500/50" />
              <span className="h-3 w-3 rounded-full bg-green-500/50" />
            </div>
            <div className="flex-1 text-center text-xs font-mono text-slate-500">neurust-cli — v2.0.1</div>
          </div>

          {/* Terminal Body */}
          <div className="flex flex-col gap-3 p-6 font-mono text-sm min-h-[300px]">
            
            {/* Line 1: Typing Input */}
            <div className="flex gap-2 text-slate-300">
              <span className="text-[#00F2EA]">➜</span>
              <span>{typedInput}</span>
              {step === 0 && (
                <motion.span
                  animate={{ opacity: [0, 1, 0] }}
                  transition={{ repeat: Infinity, duration: 0.8 }}
                  className="h-5 w-2 bg-[#FF7E5F]"
                />
              )}
            </div>

            {/* Subsequent Lines (Animated Fade In) */}
            {step >= 1 && (
              <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="text-slate-400">
                {SEQUENCE[1].text}
              </motion.div>
            )}

            {step >= 2 && (
              <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="text-[#00F2EA]">
                {SEQUENCE[2].text}
              </motion.div>
            )}

            {step >= 3 && (
              <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="text-slate-400">
                {SEQUENCE[3].text}
              </motion.div>
            )}

            {step >= 4 && (
              <motion.div initial={{ opacity: 0, x: -10 }} animate={{ opacity: 1, x: 0 }} className="text-green-400 font-bold border-t border-white/10 pt-2 mt-2">
                {SEQUENCE[4].text}
              </motion.div>
            )}
            
            {step >= 4 && (
               <div className="flex gap-2 text-slate-300 mt-2">
                <span className="text-[#00F2EA]">➜</span>
                <motion.span
                  animate={{ opacity: [0, 1, 0] }}
                  transition={{ repeat: Infinity, duration: 0.8 }}
                  className="h-5 w-2 bg-[#FF7E5F]"
                />
              </div>
            )}

          </div>
        </motion.div>
      </div>
    </section>
  );
};