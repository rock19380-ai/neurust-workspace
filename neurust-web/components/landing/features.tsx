import { CodeXml, ShieldCheck, Rocket } from 'lucide-react';

const features = [
  {
    icon: CodeXml,
    title: 'AI-Powered Scaffolding',
    description:
      'Instantly generate robust Solana program templates, CLI scaffolds, and client bindings tuned for best practices.',
  },
  {
    icon: ShieldCheck,
    title: 'Automated Security Audits',
    description:
      'Continuous AI-assisted audits surface vulnerabilities, gas inefficiencies, and upgrade risks before deployment.',
  },
  {
    icon: Rocket,
    title: 'One-Click Deployment & Management',
    description:
      'Deploy and upgrade programs with safety gates, release notes, and multi-environment rollouts from a single dashboard.',
  },
];

export function Features() {
  return (
    <section className="mx-auto max-w-6xl px-6 py-20">
      <div className="text-center">
        <p className="text-sm uppercase tracking-[0.4em] text-cyber-blue">Features</p>
        <h2 className="mt-3 text-3xl font-semibold text-white sm:text-4xl">
          Everything you need to ship secure Solana programs
        </h2>
        <p className="mt-4 text-lg text-neutral-300">
          Build faster, audit smarter, and deploy with confidence using AI copilots that understand every layer of the
          stack.
        </p>
      </div>

      <div className="mt-14 grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {features.map((feature) => {
          const Icon = feature.icon;

          return (
            <article
              key={feature.title}
              className="group rounded-2xl border border-neutral-800 bg-slate-950/40 p-6 transition hover:border-neon-rust hover:shadow-[0_0_25px_rgba(255,94,0,0.4)]"
            >
              <div className="inline-flex h-12 w-12 items-center justify-center rounded-2xl bg-slate-900">
                <Icon className="h-6 w-6 text-neon-rust transition group-hover:text-white" />
              </div>
              <h3 className="mt-5 text-xl font-semibold text-white">{feature.title}</h3>
              <p className="mt-2 text-sm text-neutral-300">{feature.description}</p>
            </article>
          );
        })}
      </div>
    </section>
  );
}
