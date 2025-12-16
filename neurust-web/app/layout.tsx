import { AuthProvider } from "./context/auth-context"; // Path မှန်အောင်စစ်ပါ
import { Navbar } from "@/components/layout/navbar";
import "./globals.css";

// Wallet Provider တွေရှိပြီးသားဖြစ်မှာပါ၊ AuthProvider ကို အထဲမှာ ထပ်ထည့်ပေးလိုက်ပါ
export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="bg-[#050505] text-white">
        {/* WalletProviders Here... */}
          <AuthProvider> 
            <Navbar />
            <main>{children}</main>
          </AuthProvider>
        {/* End WalletProviders */}
      </body>
    </html>
  );
}