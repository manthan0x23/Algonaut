import { ArenaPage } from "@/app/main/arena";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_layout/arena/$roomId")({
  component: ArenaPage,
});
