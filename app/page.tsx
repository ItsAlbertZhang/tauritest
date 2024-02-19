"use client";

import { getVersion } from "@tauri-apps/api/app";
import { useState, useEffect } from "react";

export default function Home() {
    const [version, setVersion] = useState("");

    useEffect(() => {
        getVersion().then((v) => setVersion(v));
    }, []);

    return <div>{`当前版本: ${version}`}</div>;
}
