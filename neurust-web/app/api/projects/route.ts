import { NextResponse } from "next/server";

// Rust Backend URL
const BACKEND_URL = "http://127.0.0.1:8000";

// GET: Fetch all projects for the connected wallet
export async function GET(req: Request) {
  try {
    // 1. Authenticate Request
    const walletAddress = req.headers.get("x-neurust-wallet");
    if (!walletAddress) {
      return NextResponse.json(
        { message: "Unauthorized: Wallet connection required" }, 
        { status: 401 }
      );
    }

    // 2. Call Rust Backend
    const response = await fetch(`${BACKEND_URL}/api/projects`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "x-neurust-wallet": walletAddress, // 🔥 Pass Identity to Gatekeeper
      },
      cache: "no-store",
    });

    if (!response.ok) {
      console.error("Backend Error:", response.status);
      return NextResponse.json({ projects: [] }, { status: 200 }); // Fail safe: return empty array
    }

    const data = await response.json();
    return NextResponse.json({ projects: data }); // Backend returns array, wrap it if needed

  } catch (error) {
    console.error("/api/projects GET error:", error);
    return NextResponse.json(
      { message: "Internal Server Error", projects: [] }, 
      { status: 500 }
    );
  }
}

// POST: Create a new project via Rust Backend
export async function POST(req: Request) {
  try {
    // 1. Authenticate Request
    const walletAddress = req.headers.get("x-neurust-wallet");
    if (!walletAddress) {
      return NextResponse.json({ message: "Unauthorized" }, { status: 401 });
    }

    const body = await req.json();
    const { name, description, framework } = body || {};

    if (!name || !description || !framework) {
      return NextResponse.json({ message: "Missing required fields" }, { status: 400 });
    }

    // 2. Map Framework Name (Frontend uses "Native", Backend Scaffolder might expect "Rust")
    const backendFramework = framework === "Native" ? "Rust" : framework;

    // 3. Forward to Rust Backend
    const backendRes = await fetch(`${BACKEND_URL}/api/project/create`, {
      method: "POST",
      headers: { 
        "Content-Type": "application/json",
        "x-neurust-wallet": walletAddress // 🔥 Pass Identity
      },
      body: JSON.stringify({ 
        name, 
        description, 
        framework: backendFramework 
      }),
    });

    if (!backendRes.ok) {
      const errBody = await backendRes.json().catch(() => null);
      return NextResponse.json(
        { message: errBody?.message || "Failed to create project on Neural Core" }, 
        { status: backendRes.status }
      );
    }

    const newProject = await backendRes.json();

    // 4. Return Success
    return NextResponse.json({ 
      message: "Project scaffolded successfully", 
      project: newProject 
    }, { status: 201 });

  } catch (error) {
    console.error("/api/projects POST error", error);
    return NextResponse.json({ message: "Internal Server Error" }, { status: 500 });
  }
}