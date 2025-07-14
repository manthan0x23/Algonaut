import { BackgroundBeams } from "@/components/ui/background-beams";
import { ThemeSwitch } from "@/integrations/theme/theme-switch";
import { Link, Outlet, useLocation } from "@tanstack/react-router";
import { PiGraph } from "react-icons/pi";
import { AuthOption } from "./auth";

const Layout = () => {
  const { pathname } = useLocation();

  return (
    <div className="h-screen w-screen antialiased relative">
      {pathname == "/" && <BackgroundBeams className="-z-40" />}
      <section className="h-[7%] w-full flex items-center justify-between px-6">
        <div>
          <Link to="/" reloadDocument>
            <PiGraph className="cursor-pointer" size={30} />
          </Link>
        </div>
        <div className="flex gap-5 items-center h-full  text-xs">
          <ThemeSwitch className="my-auto" showSwitch />
          <AuthOption />
        </div>
      </section>
      <section className="h-[93%] w-full bg-transparent">
        <Outlet />
      </section>
    </div>
  );
};

export default Layout;
