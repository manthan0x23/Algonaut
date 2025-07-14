import { ArenaPage } from "@/app/main/arena";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/arena/$roomId")({
  component: ArenaPage,
});
