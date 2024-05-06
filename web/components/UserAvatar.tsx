import { fetchApi } from "@/lib/api";
import { User } from "@/lib/schema";
import { Avatar, AvatarFallback } from "./ui/avatar";

export default async function UserAvatar() {
  const user = (await (await fetchApi("/users/me")).json()) as User;

  return (
    <Avatar>
      <AvatarFallback className="uppercase">{user.name[0]}</AvatarFallback>
    </Avatar>
  );
}
