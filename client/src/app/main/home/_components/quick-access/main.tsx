import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Box, PlusCircle } from "lucide-react";
import { UserRoomsWrapper } from "./table/wrap";
import { useAuthStore } from "@/store/auth-store";

type Props = {
  setCreateOpen: (v: boolean) => void;
  setJoinOpen: (v: boolean) => void;
};

export const QuickAccessCard = ({ setCreateOpen, setJoinOpen }: Props) => {
  const session = useAuthStore((state) => state.session);

  return (
    <Card className="w-1/2">
      <CardHeader>
        <CardTitle>Quick Access</CardTitle>
        <CardDescription>
          Join or create a space to get started.
        </CardDescription>
        <CardAction className="flex items-center justify-center gap-3 ">
          <Button
            disabled={!session}
            onClick={() => setJoinOpen(true)}
            size={"sm"}
            variant={"secondary"}
          >
            <Box size={14} className="mr-1" />
            Join
          </Button>
          <Button
            disabled={!session}
            onClick={() => setCreateOpen(true)}
            size={"sm"}
            variant={"outline"}
          >
            <PlusCircle size={14} className="mr-1" />
            Create
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent>{session && <UserRoomsWrapper />}</CardContent>
    </Card>
  );
};
