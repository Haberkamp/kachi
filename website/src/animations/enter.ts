import { animate, stagger } from "motion";

const START_DELAY = 0.2;
const STAGGER_DELAY = 0.12;

export function initEnterAnimation() {
  const elements = document.querySelectorAll("[data-enter]");

  if (!elements.length) return Promise.resolve();

  // Set initial state
  elements.forEach((el) => {
    (el as HTMLElement).style.opacity = "0";
    (el as HTMLElement).style.transform = "translateY(24px)";
    (el as HTMLElement).style.filter = "blur(12px)";
  });

  return new Promise<void>((resolve) => {
    setTimeout(() => {
      animate(
        elements,
        { opacity: 1, transform: "translateY(0px)", filter: "blur(0px)" },
        {
          delay: stagger(STAGGER_DELAY),
          type: "spring",
          stiffness: 400,
          damping: 30,
        }
      ).then(() => resolve());
    }, START_DELAY * 1000);
  });
}

