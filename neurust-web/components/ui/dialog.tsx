"use client";

import React from 'react';

// Props types (Optional but good for clarity)
interface DialogProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  onConfirm?: () => void;
  confirmText?: string;
  showFooter?: boolean; // Footer လိုမလို ထိန်းချုပ်ရန်
}

const Dialog = ({ 
  isOpen, 
  onClose, 
  title, 
  children, 
  onConfirm, 
  confirmText = "Confirm",
  showFooter = true 
}: DialogProps) => {
  if (!isOpen) return null;

  return (
    // Backdrop
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      
      {/* Modal Container */}
      <div className="bg-[#0a0a0a] text-white rounded-xl shadow-2xl w-full max-w-md transform transition-all scale-100 border border-white/10">
        
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-white/10">
          <h3 className="text-xl font-bold">{title}</h3>
          <button 
            onClick={onClose}
            className="text-slate-400 hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
          </button>
        </div>

        {/* Body Content */}
        <div className="p-6 text-slate-300 leading-relaxed">
          {children}
        </div>

        {/* Footer Actions (Optional) */}
        {showFooter && (
          <div className="flex justify-end gap-3 p-6 pt-0">
            <button 
              onClick={onClose}
              className="px-4 py-2 text-slate-300 hover:bg-white/10 rounded-lg font-medium transition-colors"
            >
              Cancel
            </button>
            {onConfirm && (
              <button 
                onClick={onConfirm}
                className="px-4 py-2 text-black bg-[#FF7E5F] hover:bg-[#FF7E5F]/90 rounded-lg font-medium transition-colors"
              >
                {confirmText}
              </button>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default Dialog;