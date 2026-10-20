import { api } from "./client";

interface GreetResponse {
  message: string;
}

/** `GET /api/greet`: the server's greeting for `name`. */
export async function getGreeting(name: string): Promise<string> {
  const query = new URLSearchParams({ name });
  const { message } = await api<GreetResponse>(`/greet?${query}`);
  return message;
}
