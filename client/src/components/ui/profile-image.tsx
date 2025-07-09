import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

type Props = {
  image: string;
  size?: string;
};

export const ProfileImage = ({ image, size }: Props) => {
  return (
    <Avatar>
      <AvatarImage fetchPriority="high" src={image} />
      <AvatarFallback>
        <AvatarImage fetchPriority="high" src={"default-profile.png"} />
      </AvatarFallback>
    </Avatar>
  );
};
