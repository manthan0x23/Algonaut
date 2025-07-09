import {
  GetUserRooms,
  type GetUserRoomsResponseT,
} from "@/integrations/api/rooms/get-room";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";
import { UserRoomsTable } from "./table";
import { Spinner } from "@/components/ui/spinner";
import { useState } from "react";

export const UserRoomsWrapper = () => {
  const [page, setPage] = useState(1);
  const [onlyCreated, setOnlyCreated] = useState(false);
  const [onlyJoined, setOnlyJoined] = useState(false);

  const { data, isLoading, error } = useQuery<GetUserRoomsResponseT>({
    queryKey: ["rooms", "user", page, onlyCreated, onlyJoined],
    queryFn: () =>
      GetUserRooms({
        page,
        only_created: onlyCreated,
        only_joined: onlyJoined,
      }),
  });

  if (isLoading)
    return (
      <div className="text-muted-foreground">
        <Spinner />
      </div>
    );

  if (error) {
    toast.error("Error fetching rooms");
    return <div className="text-destructive">Failed to load</div>;
  }

  if (!data) return null;

  return (
    <UserRoomsTable
      data={data.data.rooms}
      page={page}
      totalPages={data.data.total_pages}
      setPage={setPage}
      onlyCreated={onlyCreated}
      onlyJoined={onlyJoined}
      setOnlyCreated={setOnlyCreated}
      setOnlyJoined={setOnlyJoined}
    />
  );
};
