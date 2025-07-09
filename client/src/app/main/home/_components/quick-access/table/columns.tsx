// components/tables/userRoomsColumns.tsx
import { type ColumnDef } from "@tanstack/react-table";
import { Badge } from "@/components/ui/badge";
import type { GetUserRoomsResponseT } from "@/integrations/api/rooms/get-room";

export type Room = GetUserRoomsResponseT["data"]["rooms"][number];

export const userRoomsColumns: ColumnDef<Room>[] = [
  {
    accessorKey: "alias",
    header: "Title",
    cell: ({ row }) => (
      <div className="font-medium">{row.original.room.alias}</div>
    ),
  },

  {
    accessorKey: "access",
    header: "",
    cell: ({ row }) => {
      const role = row.original.role;
      let variant:
        | "default"
        | "secondary"
        | "destructive"
        | "outline" = "default";
      if (role == "editor") variant = "secondary";
      if (role == "viewer") variant = "outline";
      return (
        <Badge title={role} variant={variant}>
          {role}
        </Badge>
      );
    },
  },

  {
    accessorKey: "created_at",
    header: "Created At",
    cell: ({ row }) => (
      <span className="text-xs text-muted-foreground">
        {new Date(row.original.room.created_at).toLocaleDateString()}
      </span>
    ),
  },
];
