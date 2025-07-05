import { FlipWords } from "@/components/ui/flip-words";

const features = [
  "Instant Setup: Spin up a new arena in one click.",
  "Real‑Time Sync: Watch edits happen live, no lag.",
  "AI‑Driven Hints: Smart suggestions when you’re stuck.",
  "Single‑File Focus: Everything you need to solve algos—nothing you don’t.",
];

const writingText =
  "Join private coding arenas and solve algorithms collaboratively. One file. Real-time editing. No setup overhead.";

export const HomePage = () => {
  return (
    <div className="flex h-full w-full text-foreground">
     <section className="w-1/2 h-full px-[10vw] flex flex-col justify-center">
        <h1>Algonaut</h1>
        <p className="mt-2 font-normal text-muted-foreground">
          Sharpen your algorithm skills—together.
        </p>

        <p className="mt-4">{writingText}</p>

        <FlipWords words={features} className="p-0 mt-6" />
      </section>

      {/* Right Panel */}
      <section />
    </div>
  );
};
