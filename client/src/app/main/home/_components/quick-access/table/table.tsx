import {
  getCoreRowModel,
  useReactTable,
  flexRender,
} from "@tanstack/react-table";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { userRoomsColumns } from "./columns";
import type { GetUserRoomsResponseT } from "@/integrations/api/rooms/get-room";
import { RoomTableControls } from "./controls";
import { ArrowRight } from "lucide-react";
import { useRouter } from "@tanstack/react-router";

type Room = GetUserRoomsResponseT["data"]["rooms"][number];

interface UserRoomsTableProps {
  data: Room[];
  page: number;
  totalPages: number;
  setPage: (p: number) => void;
  onlyCreated: boolean;
  onlyJoined: boolean;
  setOnlyCreated: (val: boolean) => void;
  setOnlyJoined: (val: boolean) => void;
}

export const UserRoomsTable = ({
  data,
  page,
  totalPages,
  onlyCreated,
  onlyJoined,
  setPage,
  setOnlyCreated,
  setOnlyJoined,
}: UserRoomsTableProps) => {
  const table = useReactTable({
    data,
    columns: userRoomsColumns,
    getCoreRowModel: getCoreRowModel(),
  });
  const router = useRouter();

  const totalRows = 8;
  const rowModel = table.getRowModel().rows;
  const paddedRows = [...rowModel];
  while (paddedRows.length < totalRows) {
    paddedRows.push(undefined as any);
  }

  return (
    <div className="space-y-4">
      <div className="rounded-md border overflow-x-auto">
        <Table>
          <TableBody>
            {paddedRows.map((row, index) =>
              row ? (
                <TableRow
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    router.navigate({
                      to: `/arena/${row.original.room.id}`,
                      search: {
                        tab: "space",
                      },
                      resetScroll: true,
                    });
                  }}
                  key={row.id}
                  className="group hover:bg-muted/40 cursor-pointer relative"
                >
                  {row.getVisibleCells().map((cell, idx, arr) => (
                    <TableCell
                      key={cell.id}
                      className={idx === arr.length - 1 ? "relative pr-6" : ""}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                      {idx === arr.length - 1 && (
                        <span className="absolute right-1 top-1/2 -translate-y-1/2 text-primary pr-3 opacity-0 group-hover:opacity-100 duration-200 transition-all">
                          <ArrowRight size={15} />
                        </span>
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ) : (
                <TableRow
                  key={`empty-${index}`}
                  className="hover:bg-transparent border-none"
                >
                  {userRoomsColumns.map((_, colIdx) => (
                    <TableCell
                      key={`empty-${index}-${colIdx}`}
                      className="border-none"
                    >
                      &nbsp;
                    </TableCell>
                  ))}
                </TableRow>
              )
            )}
          </TableBody>
        </Table>
      </div>

      <RoomTableControls
        page={page}
        totalPages={totalPages}
        setPage={setPage}
        onlyCreated={onlyCreated}
        onlyJoined={onlyJoined}
        setOnlyCreated={setOnlyCreated}
        setOnlyJoined={setOnlyJoined}
      />
    </div>
  );
};
