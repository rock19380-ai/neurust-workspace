import { NextResponse } from "next/server";
import fs from "fs/promises";
import path from "path";

const DATA_PATH = path.join(process.cwd(), "data", "projects.json");

async function ensureStore() {
  try {
    await fs.access(DATA_PATH);
  } catch {
    await fs.mkdir(path.dirname(DATA_PATH), { recursive: true });
    await fs.writeFile(DATA_PATH, JSON.stringify([]), "utf-8");
  }
}

export async function GET(_: Request, { params }: { params: { id: string } }) {
  try {
    await ensureStore();
    const raw = await fs.readFile(DATA_PATH, "utf-8");
    const projects = JSON.parse(raw || "[]") as any[];
    const project = projects.find((p) => p.id === params.id);

    if (!project) {
      return NextResponse.json({ message: "Not found" }, { status: 404 });
    }

    return NextResponse.json({ project });
  } catch (error) {
    console.error(`/api/projects/${params.id} GET error`, error);
    return NextResponse.json({ message: "Internal Server Error" }, { status: 500 });
  }
}

export async function DELETE(_: Request, { params }: { params: { id: string } }) {
  try {
    await ensureStore();
    const raw = await fs.readFile(DATA_PATH, "utf-8");
    const projects = JSON.parse(raw || "[]") as any[];
    const projectIndex = projects.findIndex((p) => p.id === params.id);

    if (projectIndex === -1) {
      return NextResponse.json({ message: "Not found" }, { status: 404 });
    }

    const project = projects[projectIndex];
    const projectPath: string | undefined = project?.path;

    // Call backend to delete folder if path exists
    if (projectPath) {
      const backendRes = await fetch("http://127.0.0.1:8000/api/project/delete", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ path: projectPath }),
      });

      if (!backendRes.ok) {
        const errBody = await backendRes.json().catch(() => null);
        const message = errBody?.message || "Failed to delete project directory";
        return NextResponse.json({ message }, { status: 502 });
      }
    }

    // Remove from local store
    projects.splice(projectIndex, 1);
    await fs.writeFile(DATA_PATH, JSON.stringify(projects, null, 2), "utf-8");

    return NextResponse.json({ message: "ok" });
  } catch (error) {
    console.error(`/api/projects/${params.id} DELETE error`, error);
    return NextResponse.json({ message: "Internal Server Error" }, { status: 500 });
  }
}
