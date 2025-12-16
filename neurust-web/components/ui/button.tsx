import { cva, type VariantProps } from "class-variance-authority";
import { forwardRef } from "react";
import type { ButtonHTMLAttributes } from "react";
import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center rounded-md text-sm font-semibold transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none",
  {
    variants: {
      variant: {
        default: "bg-primary text-black hover:bg-primary/90",
        ghost: "bg-transparent text-white hover:bg-slate-800",
        outline: "border border-slate-700 bg-transparent text-white hover:bg-slate-800",
        neonRust:
          "border border-[#FF7E5F] bg-[#0b0b0b]/70 text-[#FF7E5F] shadow-[0_0_15px_rgba(255,126,95,0.5)] duration-300 ease-in-out hover:bg-[rgba(255,126,95,0.08)] hover:scale-[1.02] hover:shadow-[0_0_25px_rgba(255,126,95,0.8)] focus-visible:ring-[#FF7E5F]/60 focus-visible:ring-offset-0",
      },
      size: {
        default: "h-11 px-6 rounded-md",
        sm: "h-9 px-4 text-sm",
        lg: "h-12 px-8 text-base",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
);

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & VariantProps<typeof buttonVariants>;

const Button = forwardRef<HTMLButtonElement, ButtonProps>(({ className, variant, size, ...props }, ref) => (
  <button
    className={cn(buttonVariants({ variant, size, className }))}
    ref={ref}
    {...props}
  />
));

Button.displayName = "Button";

export { Button, buttonVariants };
