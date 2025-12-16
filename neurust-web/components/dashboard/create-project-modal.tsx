"use client";

import { useState } from "react";
import { Copy, Terminal, Check } from "lucide-react";
import { Button } from "@/components/ui/button"; // သင့် Button component path အတိုင်းထားပါ
import Dialog from "@/components/ui/dialog"; // (Import path မှန်အောင်စစ်ပါ)

export default function CreateProjectModal() {
  const [isOpen, setIsOpen] = useState(false);
  const [copied, setCopied] = useState(false);
  const command = "neurust new my_dapp --framework anchor";

  const handleCopy = () => {
    navigator.clipboard.writeText(command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <>
      {/* Trigger Button - Modal အပြင်မှာထားရပါမယ် */}
      <Button 
        onClick={() => setIsOpen(true)}
        variant="default" 
        className="bg-[#FF7E5F] text-black hover:bg-[#FF7E5F]/90"
      >
        <Terminal className="mr-2 h-4 w-4" />
        New Project
      </Button>

      {/* Custom Dialog Component Usage */}
      <Dialog
        isOpen={isOpen}
        onClose={() => setIsOpen(false)}
        title="Create via CLI"
        showFooter={false} // Footer ခလုတ်တွေမလိုချင်ရင် false ပေးပါ
      >
        <div className="space-y-4">
          <p className="text-sm text-slate-400">
            To ensure full file system access and security, please create new projects directly from your terminal.
          </p>
          
          <div className="relative rounded-lg bg-black/50 p-4 border border-white/10 group">
            <code className="text-sm font-mono text-[#00F2EA]">{command}</code>
            <button
              onClick={handleCopy}
              className="absolute right-3 top-3 text-slate-400 hover:text-white transition-colors"
              title="Copy to clipboard"
            >
              {copied ? <Check size={16} className="text-emerald-400" /> : <Copy size={16} />}
            </button>
          </div>

          <div className="text-xs text-slate-500">
            After running this, the project will automatically sync with your dashboard.
          </div>
        </div>
      </Dialog>
    </>
  );
}