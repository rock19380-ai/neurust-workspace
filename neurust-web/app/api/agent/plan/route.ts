import { NextResponse } from "next/server";

export async function POST(req: Request) {
  try {
    const body = await req.json();
    
    // Frontend ကနေ ပို့လိုက်တဲ့ Wallet Address ကို Header ထဲကနေ ယူမယ်
    const walletAddress = req.headers.get("x-neurust-wallet");

    if (!walletAddress) {
      return NextResponse.json(
        { message: "Wallet authentication required" },
        { status: 401 }
      );
    }

    // Rust Backend ကို လှမ်းခေါ်မယ် (Wallet Header ကို လက်ဆင့်ကမ်းမယ်)
    const response = await fetch("http://127.0.0.1:8000/api/agent/plan", {
      method: "POST",
      headers: { 
        "Content-Type": "application/json",
        "x-neurust-wallet": walletAddress // 🔥 Backend Gatekeeper အတွက် အဓိကသော့ချက်
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      const errorData = await response.json();
      return NextResponse.json(errorData, { status: response.status });
    }

    const data = await response.json();
    return NextResponse.json(data);
    
  } catch (error) {
    console.error("Neurust Brain Error:", error);
    return NextResponse.json(
      { message: "Internal Server Error" },
      { status: 500 }
    );
  }
}