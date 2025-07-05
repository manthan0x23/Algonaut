import { HomePage } from "@/app/main/home";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_layout/")({
  component: HomePage,
});
