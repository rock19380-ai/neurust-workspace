"use client";

import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import { Send, Loader2, Wallet } from "lucide-react";

export function ChatInterface() {
  const { publicKey, connected } = useWallet();
  const [prompt, setPrompt] = useState("");
  const [response, setResponse] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!prompt.trim() || !publicKey) return;

    setLoading(true);
    setResponse(null);

    try {
      // အခုနက ဆောက်လိုက်တဲ့ API route ကို လှမ်းခေါ်မယ်
      const res = await fetch("/api/agent/plan", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-neurust-wallet": publicKey.toString(), // 🔥 Wallet Address ပို့မယ်
        },
        body: JSON.stringify({ prompt }),
      });

      const data = await res.json();
      setResponse(data);
    } catch (error) {
      console.error("Error:", error);
      setResponse({ message: "Failed to connect to Neurust Brain." });
    } finally {
      setLoading(false);
    }
  };

  if (!connected) {
    return (
      <div className="flex flex-col items-center justify-center p-10 text-center border border-white/10 rounded-xl bg-white/5 mt-8">
        <Wallet className="w-12 h-12 text-slate-500 mb-4" />
        <h3 className="text-xl font-bold text-white">Wallet Not Connected</h3>
        <p className="text-slate-400 mt-2">Please connect your Phantom wallet to access Neurust AI.</p>
      </div>
    );
  }

  return (
    <div className="w-full max-w-4xl mx-auto space-y-6 mt-8">
      {/* Output Display */}
      {response && (
        <div className="p-6 rounded-xl bg-[#111] border border-white/10 text-slate-300 font-mono whitespace-pre-wrap shadow-lg">
          <div className="text-xs uppercase tracking-widest text-[#FF7E5F] mb-2 border-b border-white/10 pb-2">Neurust Output</div>
          {/* JSON ကို ဖတ်လို့ကောင်းအောင် ပြမယ် */}
          {typeof response === 'string' ? response : JSON.stringify(response, null, 2)}
        </div>
      )}

      {/* Input Area */}
      <form onSubmit={handleSubmit} className="relative group">
        <div className="absolute -inset-0.5 bg-gradient-to-r from-[#FF7E5F] to-[#00F2EA] rounded-xl opacity-20 group-hover:opacity-40 transition duration-500 blur"></div>
        <div className="relative flex">
          <input
            type="text"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Ask Neurust (e.g., 'Plan a Solana NFT marketplace')..."
            className="w-full bg-black border border-white/10 rounded-xl px-6 py-4 pr-16 text-white placeholder:text-slate-500 focus:outline-none focus:ring-1 focus:ring-[#FF7E5F]/50 transition-all"
          />
          <button
            type="submit"
            disabled={loading || !prompt.trim()}
            className="absolute right-2 top-2 p-2 bg-[#FF7E5F] rounded-lg text-black hover:bg-[#FF7E5F]/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? <Loader2 className="animate-spin" size={20} /> : <Send size={20} />}
          </button>
        </div>
      </form>
      
      <div className="text-center text-xs text-slate-600">
        Connected as: <span className="font-mono text-[#FF7E5F]">{publicKey?.toString().slice(0, 6)}...{publicKey?.toString().slice(-4)}</span>
      </div>
    </div>
  );
}