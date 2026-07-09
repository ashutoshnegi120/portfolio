(function () {
  function bootAnimations() {
    if (window.gsap) {
      gsap.from(".hud-panel, .dialogue-box", {
        y: 16,
        opacity: 0,
        duration: 0.7,
        stagger: 0.08,
        ease: "steps(6)",
      });

      gsap.to(".hero-sprite", {
        y: -8,
        duration: 0.85,
        repeat: -1,
        yoyo: true,
        ease: "steps(3)",
      });

      gsap.to(".cloud", {
        x: 22,
        duration: 3,
        repeat: -1,
        yoyo: true,
        ease: "steps(5)",
        stagger: 0.4,
      });
    }

    if (window.anime) {
      anime({
        targets: ".coin",
        translateY: [-5, 5],
        rotate: "1turn",
        duration: 1300,
        delay: anime.stagger(170),
        loop: true,
        direction: "alternate",
        easing: "steps(4)",
      });

      anime({
        targets: ".scanline-glow",
        opacity: [0.2, 0.55],
        duration: 900,
        loop: true,
        direction: "alternate",
        easing: "linear",
      });
    }
  }

  function wireControls() {
    document.querySelectorAll("[data-warp]").forEach((button) => {
      button.addEventListener("click", () => {
        const target = document.querySelector(button.getAttribute("data-warp"));
        if (target) target.scrollIntoView({ behavior: "smooth", block: "start" });
      });
    });

    document.querySelectorAll(".repo-card").forEach((card) => {
      card.addEventListener("pointermove", (event) => {
        const rect = card.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        card.style.setProperty("--mx", `${x}px`);
        card.style.setProperty("--my", `${y}px`);

        if (card.classList.contains("flipped")) {
          card.style.setProperty("--tilt-x", "0deg");
          card.style.setProperty("--tilt-y", "0deg");
          return;
        }

        const tiltY = (x / rect.width - 0.5) * 9;
        const tiltX = (0.5 - y / rect.height) * 7;
        card.style.setProperty("--tilt-x", `${tiltX}deg`);
        card.style.setProperty("--tilt-y", `${tiltY}deg`);
      });

      card.addEventListener("pointerleave", () => {
        card.style.setProperty("--tilt-x", "0deg");
        card.style.setProperty("--tilt-y", "0deg");
      });

      card.addEventListener("keydown", (event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          card.click();
        }
      });
    });

    document.querySelectorAll(".skill-coin").forEach((coin) => {
      coin.addEventListener("pointermove", (event) => {
        const rect = coin.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;
        coin.style.setProperty("--coin-x", `${x}px`);
        coin.style.setProperty("--coin-y", `${y}px`);
      });
    });
  }

  window.startArcadePortfolio = function () {
    requestAnimationFrame(() => {
      bootAnimations();
      wireControls();
    });
  };
})();
