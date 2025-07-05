import { BackgroundBeams } from "@/components/ui/background-beams";
import { Button } from "@/components/ui/button";
import { ThemeSwitch } from "@/integrations/theme/theme-switch";
import { Outlet, useLocation } from "@tanstack/react-router";
import { Binary } from "lucide-react";
import { FcGoogle } from "react-icons/fc";

const Layout = () => {
  const { pathname } = useLocation();

  return (
    <div className="h-screen w-screen antialiased relative">
      {pathname == "/" && <BackgroundBeams className="-z-40" />}{" "}
      <section className="h-[7%] w-full flex items-center justify-between px-6 bg-background">
        <div>
          <Binary />
        </div>
        <div className="flex gap-5 items-center  text-xs">
          <ThemeSwitch />
          <Button size={"sm"} variant={"outline"}>
            <FcGoogle />
            Login with Google
          </Button>
        </div>
      </section>
      <section className="h-[93%] w-full bg-transparent">
        <Outlet />
      </section>
    </div>
  );
};

export default Layout;
