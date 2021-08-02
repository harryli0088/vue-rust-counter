export default async function handleResponse(res: Response) {
  if(!res.ok) {
    throw new Error(await res.text())
  }
  return res
}