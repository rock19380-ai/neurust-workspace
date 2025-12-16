import { Features } from "@/components/landing/features";
import { Hero } from "@/components/landing/hero";
import { Navbar } from "@/components/layout/navbar";

export default function Page() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-black via-[#050505] to-[#020202] text-white">
      <Hero />
      <Features />
    </div>
  );
}
