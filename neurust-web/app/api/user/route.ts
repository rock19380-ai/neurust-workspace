import { NextResponse } from "next/server";

export async function GET(req: Request) {
  const walletAddress = req.headers.get("x-neurust-wallet");
  if (!walletAddress) return NextResponse.json({ message: "Unauthorized" }, { status: 401 });

  try {
    const res = await fetch("http://127.0.0.1:8000/api/user/me", {
      headers: { "x-neurust-wallet": walletAddress },
    });
    if (!res.ok) return NextResponse.json({ credits: 0 }, { status: 200 }); // Default to 0 on error
    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ credits: 0 }, { status: 500 });
  }
}