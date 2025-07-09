import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  ChevronsLeft,
  ChevronLeft,
  ChevronRight,
  ChevronsRight,
  Filter,
} from "lucide-react";

export const RoomTableControls = ({
  page,
  totalPages,
  setPage,
  onlyCreated,
  onlyJoined,
  setOnlyCreated,
  setOnlyJoined,
}: {
  page: number;
  totalPages: number;
  setPage: (p: number) => void;
  onlyCreated: boolean;
  onlyJoined: boolean;
  setOnlyCreated: (b: boolean) => void;
  setOnlyJoined: (b: boolean) => void;
}) => {
  return (
    <div className="flex flex-col sm:flex-row sm:justify-between items-center gap-4">
      {/* Filter Dropdown */}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="sm">
            <Filter className="mr-2 h-4 w-4" />
            Filter
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-48" align="start">
          <DropdownMenuCheckboxItem
            checked={onlyCreated}
            onCheckedChange={(val) => {
              setOnlyCreated(val);
              if (val) setOnlyJoined(false);
              if (!val && !onlyJoined) setOnlyCreated(false);
              setPage(1);
            }}
          >
            Only Created
          </DropdownMenuCheckboxItem>
          <DropdownMenuCheckboxItem
            checked={onlyJoined}
            onCheckedChange={(val) => {
              setOnlyJoined(val);
              if (val) setOnlyCreated(false);
              if (!val && !onlyCreated) setOnlyJoined(false);
              setPage(1);
            }}
          >
            Only Joined
          </DropdownMenuCheckboxItem>
          <DropdownMenuCheckboxItem
            checked={!onlyCreated && !onlyJoined}
            onCheckedChange={() => {
              setOnlyCreated(false);
              setOnlyJoined(false);
              setPage(1);
            }}
          >
            All
          </DropdownMenuCheckboxItem>
        </DropdownMenuContent>
      </DropdownMenu>

      {/* Pagination */}
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPage(1)}
          disabled={page === 1}
        >
          <ChevronsLeft className="h-4 w-4" />
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPage(page - 1)}
          disabled={page === 1}
        >
          <ChevronLeft className="h-4 w-4" />
        </Button>
        <span className="text-sm text-muted-foreground">
          Page {page} of {totalPages}
        </span>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPage(page + 1)}
          disabled={page === totalPages}
        >
          <ChevronRight className="h-4 w-4" />
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPage(totalPages)}
          disabled={page === totalPages}
        >
          <ChevronsRight className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
};
