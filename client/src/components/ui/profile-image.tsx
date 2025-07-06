import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

type Props = {
  image: string;
  size?: string;
};

export const ProfileImage = ({ image, size }: Props) => {
  return (
    <Avatar>
      <AvatarImage sizes={size} src={image} />
      <AvatarFallback>
        <AvatarImage src={"default-icon.jpg"} />
      </AvatarFallback>
    </Avatar>
  );
};
