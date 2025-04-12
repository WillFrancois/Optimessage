import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
  const response: string = await fetch('http://localhost:1337/optimessage/messages').then((response) => {
    return response.text();
  });

  let messageList: string[] = response.split("\n")
  messageList = messageList.slice(0, -1)

  return { messageList };
}
