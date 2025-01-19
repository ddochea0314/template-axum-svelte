<script>
  let links = [
    { href: '/', text: 'Home' },
    { href: '/mypage', text: 'My Page' },
    { href: '/movie', text: 'Movie' }
  ];
</script>

<nav>
  <ul>
    {#each links as link}
      <li><a href={link.href}>{link.text}</a></li>
    {/each}
  </ul>
</nav>

<style>
  nav {
    background-color: #333;
    padding: 1rem;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    display: inline;
    margin-right: 1rem;
  }

  a {
    color: white;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>