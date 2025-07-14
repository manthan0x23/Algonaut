import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ProfileImage } from "@/components/ui/profile-image";
import { Env } from "@/lib/env";
import { useAuthStore } from "@/store/auth-store";
import { ChevronDown } from "lucide-react";
import { FcGoogle } from "react-icons/fc";

export const AuthOption = () => {
  const { session, isLoading } = useAuthStore((state) => state);

  const loginWithGoogle = (
    e: React.MouseEvent<HTMLButtonElement, MouseEvent>
  ) => {
    e.preventDefault();
    e.stopPropagation();

    window.location.assign(`${Env.server_url}/api/auth/google/login`);
  };

  return (
    <div className="h-full w-auto flex">
      {session ? (
        <DropdownMenu>
          <DropdownMenuTrigger className="p-0 w-full outline-none">
            <div className="text-sm h-full font-medium flex items-center gap-3 cursor-pointer px-4 hover:scale-[1.02] transform-all">
              <p>{session.user.name ?? session.user.email}</p>
              <ProfileImage
                image={session.user.avatar_url}
                label={session.user.name ?? session.user.email}
              />
              <ChevronDown size={13} className="text-muted-foreground" />
            </div>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-full " align="end">
            <DropdownMenuLabel>My Account</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>Profile</DropdownMenuItem>
            <DropdownMenuItem>Credits</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      ) : (
        <Button
          size={"sm"}
          variant={"outline"}
          onClick={loginWithGoogle}
          className="my-auto outline"
          disabled={isLoading}
        >
          <FcGoogle />
          Login with Google
        </Button>
      )}
    </div>
  );
};
