import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

type Props = {
  image: string;
  size?: string;
  label?: string;
};

export const ProfileImage = ({ image, size, label }: Props) => {
  return (
    <Avatar>
      <AvatarImage fetchPriority="high" src={image} />
      <AvatarFallback>{label ? label[0] : "U"} </AvatarFallback>
    </Avatar>
  );
};
