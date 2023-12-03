import Image from "next/image";
import styles from "./page.module.css";
import background from "../public/Super Vase.png";

export default function Home() {
  return (
    <main className={styles.main}>
      <Image
        src={background}
        className="background-img"
        alt="A vase of tall colorful flowers"
      />
      <header className="header">
        <h1 className="title">JET PHAM</h1>
      </header>
    </main>
  );
}
