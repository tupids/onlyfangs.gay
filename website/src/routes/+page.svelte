<script lang="ts">
  import sodaImageUrl from '$lib/images/soda.webp?url';
  import tyler1ImageUrl from '$lib/images/tyler1.webp?url';
  import lacariImageUrl from '$lib/images/lacari.webp?url';
</script>

<div class="container">
  <h1>OnlyFangs Signup</h1>

  <button class="twitch-button">Login with Twitch</button>
  <img src={sodaImageUrl} alt="fasd" class="floating-image image1" />

  <img src={tyler1ImageUrl} alt="fads" class="floating-image image2" />

  <img src={lacariImageUrl} alt="fads" class="floating-image image3" />
</div>

<style>
  .container {
    min-height: 100vh;
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
  }

  h1 {
    font-size: 3rem;
    margin-bottom: 2rem;
    color: #ff6b6b;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
  }

  .floating-image {
    position: absolute;
    border-radius: 10px;
  }

  .image1 {
    top: 15%;
    left: 15%;
    transform: rotate(-5deg);
  }

  .image2 {
    bottom: 20%;
    right: 15%;
    transform: rotate(7deg);
  }

  .image3 {
    bottom: 10%;
    left: 15%;
    transform: rotate(5deg);
  }

  .twitch-button {
    background-color: #9146ff;
    color: white;
    border: none;
    padding: 1rem 2rem;
    font-size: 1.2rem;
    border-radius: 5px;
    cursor: pointer;
    transition: background-color 0.2s;
    margin-top: 2rem;
    z-index: 1000;
  }

  .twitch-button:hover {
    background-color: #7a2bff;
  }

  @media (max-width: 768px) {
    .floating-image {
      position: static;
      margin: 1rem 0;
      transform: none;
      max-width: 80%;
      height: auto;
    }

    h1 {
      font-size: 2rem;
    }
  }
</style>
