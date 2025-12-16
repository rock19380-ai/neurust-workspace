"use client";

import { useState } from "react";
import { Buffer } from "buffer";
import { useRouter } from "next/navigation";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { useWallet } from "@solana/wallet-adapter-react";
import { Loader2 } from "lucide-react"; // Loading icon ထည့်ရန် (Optional)

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

type AuthStatus = "idle" | "loading" | "error" | "success";

export default function LoginPage() {
  const router = useRouter();
  const { publicKey, signMessage, connected } = useWallet();
  const [deviceCode, setDeviceCode] = useState("");
  const [status, setStatus] = useState<AuthStatus>("idle");
  const [feedback, setFeedback] = useState("");

  const isLoading = status === "loading";

  // Code ရိုက်ထည့်လိုက်တာနဲ့ အကြီးစာလုံး (UPPERCASE) အလိုလိုပြောင်းပေးမယ်
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setDeviceCode(e.target.value.toUpperCase());
  };

  const handleVerify = async () => {
    if (!connected || !publicKey || !signMessage) {
      setStatus("error");
      setFeedback("Please connect your wallet first.");
      return;
    }

    const trimmed = deviceCode.trim();

    if (!trimmed || trimmed.length < 4) {
      setStatus("error");
      setFeedback("Invalid Device Code.");
      return;
    }

    try {
      setStatus("loading");
      setFeedback("Requesting wallet signature...");

      // 1. Sign Message to prove ownership
      const message = `Login to Neurust CLI.\nDevice Code: ${trimmed}\nWallet: ${publicKey.toBase58()}`;
      const encoded = new TextEncoder().encode(message);
      const signature = await signMessage(encoded);

      setFeedback("Verifying with server...");

      const payload = {
        user_code: trimmed,
        wallet_address: publicKey.toBase58(),
        message: message, // Backend needs original message to verify signature
        signature: Buffer.from(signature).toString("base64"),
      };

      // 2. Send to Backend
      const response = await fetch("http://localhost:8000/api/auth/device/verify", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => null);
        throw new Error(errorData?.message ?? "Device verification failed.");
      }

      // 3. Success
      setStatus("success");
      setFeedback("Success! You can now close this tab.");
      
      // Redirect to dashboard (Optional, since this flow is mainly for CLI)
      setTimeout(() => router.push("/dashboard"), 2000);

    } catch (error: any) {
      console.error(error);
      setStatus("error");
      setFeedback(error.message || "Verification failed. Please try again.");
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-[#050505] px-4 py-12 relative overflow-hidden">
      
      {/* Background Ambience */}
      <div className="absolute top-[-10%] left-[-10%] w-[500px] h-[500px] bg-[#FF7E5F]/10 rounded-full blur-[120px] pointer-events-none" />
      <div className="absolute bottom-[-10%] right-[-10%] w-[500px] h-[500px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none" />

      <div className="w-full max-w-lg space-y-8 rounded-3xl border border-white/10 bg-[#0a0a0a]/80 p-8 md:p-10 shadow-2xl backdrop-blur-xl relative z-10">
        
        {/* Header */}
        <div className="space-y-4 text-center">
          <div className="inline-block rounded-full bg-white/5 px-3 py-1 border border-white/10">
             <p className="text-[10px] font-bold uppercase tracking-[0.3em] text-[#00F2EA]">Device Flow</p>
          </div>
          <h1 className="text-3xl font-bold text-white sm:text-4xl tracking-tight">Authorize CLI</h1>
          <p className="text-sm text-slate-400 leading-relaxed">
            Please enter the code displayed in your terminal <br/> to connect your wallet securely.
          </p>
        </div>

        {/* Input Section */}
        <div className="space-y-2">
          <label className="text-xs font-bold uppercase tracking-widest text-slate-500 ml-1">
            Device Code
          </label>
          <Input
            className="h-14 text-center text-2xl font-mono font-bold tracking-[0.3em] uppercase 
                       bg-black/50 border-white/10 text-white placeholder:text-white/10 
                       focus:border-[#FF7E5F] focus:ring-[#FF7E5F]/20 transition-all rounded-xl"
            placeholder="ABCD-1234"
            value={deviceCode}
            onChange={handleInputChange}
            maxLength={9} // Assuming format XXXX-XXXX
          />
        </div>

        {/* Wallet Section */}
        <div className="space-y-2">
           <label className="text-xs font-bold uppercase tracking-widest text-slate-500 ml-1">
            Wallet Connection
          </label>
          <div className="wallet-adapter-button-trigger w-full">
            <WalletMultiButton style={{
                width: '100%',
                justifyContent: 'center',
                background: 'rgba(255,255,255,0.05)',
                border: '1px solid rgba(255,255,255,0.1)',
                borderRadius: '12px',
                height: '50px',
                fontFamily: 'monospace',
                fontWeight: 'bold'
            }} />
          </div>
        </div>

        {/* 🔥 Verify Button (High Contrast) */}
        <Button
          className={`w-full h-14 text-base font-bold tracking-widest uppercase transition-all duration-300 rounded-xl
            ${status === 'success' 
               ? "bg-emerald-500 hover:bg-emerald-600 text-white shadow-[0_0_30px_rgba(16,185,129,0.4)]" 
               : "bg-[#FF7E5F] hover:bg-[#ff6b4a] text-black shadow-[0_0_20px_rgba(255,126,95,0.4)] hover:shadow-[0_0_30px_rgba(255,126,95,0.6)]"
            }
          `}
          onClick={handleVerify}
          disabled={!connected || isLoading || !deviceCode}
        >
          {isLoading ? (
            <div className="flex items-center gap-2">
              <Loader2 className="h-4 w-4 animate-spin" />
              Verifying...
            </div>
          ) : status === 'success' ? (
            "Verified Successfully"
          ) : (
            "Verify & Login"
          )}
        </Button>

        {/* Status Message */}
        {feedback && (
          <div className={`p-3 rounded-lg text-center text-sm font-medium border ${
            status === "error" 
                ? "bg-red-500/10 border-red-500/20 text-red-400" 
                : status === "success"
                ? "bg-emerald-500/10 border-emerald-500/20 text-emerald-400"
                : "bg-blue-500/10 border-blue-500/20 text-blue-400"
          }`}>
            {feedback}
          </div>
        )}
      </div>
    </div>
  );
}