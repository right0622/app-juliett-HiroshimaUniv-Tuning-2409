import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "../css/global.scss";
import { AuthProvider } from "@/context/AuthContext";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "HiroshimaUniv-Tuning-2409"
};
export const viewport = "width=device-width, initial-scale=1";

export default function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={inter.className}>
        <AuthProvider>{children}</AuthProvider>
      </body>
    </html>
  );
}
