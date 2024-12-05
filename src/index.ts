import { Elysia } from "elysia";
import { supabase } from "./lib/supabase";
import crypto from "crypto";

// Function to generate short code
const generateShortCode = () => {
  return crypto.randomBytes(2).toString("hex");
};

const app = new Elysia()
  .post("/url", async ({ body }) => {
    const { url } = body as { url: string };

    if (!url) {
      throw new Error("URL is required");
    }

    const shortCode = generateShortCode();

    const { data, error } = await supabase
      .from("urls")
      .insert([
        { original_url: url, short_code: shortCode },
      ])
      .select()
      .single();

    if (error) throw new Error(error.message);
    return data;
  })
  .get("/urls", async () => {
    const { data, error } = await supabase
      .from("urls")
      .select("*")
      .order("created_at", { ascending: false });

    if (error) throw new Error(error.message);
    return data;
  })
  .get("/:shortCode", async ({ params, redirect }) => {
    const { shortCode } = params;

    const { data, error } = await supabase
      .from("urls")
      .select("*")
      .eq("short_code", shortCode)
      .single();

    if (error || !data) {
      throw new Error("URL not found");
    }

    // Fire and forget the increment_clicks RPC
    Promise.resolve(supabase.rpc('increment_clicks', { url_id: data.id }));

    return redirect(data.original_url);
  })
  .listen(process.env.PORT ?? 3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`,
);
