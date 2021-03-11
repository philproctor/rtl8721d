<template>
  <div id="layout">

    <!-- For small views -->
    <a href="#menu" class="menu-link">
      <span></span>
    </a>

    <!-- Side menu -->
    <div id="menu">
      <div class="pure-menu">
        <a class="pure-menu-heading">CORVUS</a>

        <!-- Menu Entries -->
        <ul class="pure-menu-list">
          <li class="pure-menu-item">
            <router-link class="pure-menu-link" to="/overview" active-class="pure-menu-selected">test</router-link>
          </li>
          <li class="pure-menu-item">
            <router-link class="pure-menu-link" to="/notfound" active-class="pure-menu-selected">other</router-link>
          </li>
        </ul>

      </div>
    </div>

    <!-- Main viewport -->
    <div id="main">
      <div class="header">
        <h1>About</h1>
        <h2>other stuff</h2>
      </div>
      <router-view />
    </div>

  </div>
</template>

<script>
export default {
  name: 'App',
}
</script>

<style>
body {
  color: #777;
  background-color: #333;
}

.button-primary {
  color: white;
  border-radius: 4px;
  text-shadow: 0 1px 1px rgba(0, 0, 0, 0.2);
}

.button-primary {
  background: rgb(76, 66, 221);
}

.pure-img-responsive {
  max-width: 100%;
  height: auto;
}

/*
Add transition to containers so they can push in and out.
*/
#layout,
#menu,
.menu-link {
  -webkit-transition: all 0.2s ease-out;
  -moz-transition: all 0.2s ease-out;
  -ms-transition: all 0.2s ease-out;
  -o-transition: all 0.2s ease-out;
  transition: all 0.2s ease-out;
}

/*
This is the parent `<div>` that contains the menu and the content area.
*/
#layout {
  position: relative;
  left: 0;
  padding-left: 0;
}
#layout.active #menu {
  left: 150px;
  width: 150px;
}

#layout.active .menu-link {
  left: 150px;
}
/*
The content `<div>` is where all your content goes.
*/
.content {
  margin: 0 auto;
  padding: 0 2em;
  max-width: 800px;
  margin-bottom: 50px;
  line-height: 1.6em;
}

.header {
  margin: 0;
  color: #aaa;
  text-align: center;
  padding: 2.5em 2em 0;
  border-bottom: 1px solid #eee;
}
.header h1 {
  margin: 0.2em 0;
  font-size: 3em;
  font-weight: 300;
}
.header h2 {
  font-weight: 300;
  color: #ccc;
  padding: 0;
  margin-top: 0;
}

.content-subhead {
  margin: 50px 0 20px 0;
  font-weight: 300;
  color: #888;
}

/*
The `#menu` `<div>` is the parent `<div>` that contains the `.pure-menu` that
appears on the left side of the page.
*/

#menu {
  margin-left: -150px; /* "#menu" width */
  width: 150px;
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  z-index: 1000; /* so the menu or its navicon stays above all content */
  background: #191818;
  overflow-y: auto;
}
/*
    All anchors inside the menu should be styled like this.
    */
#menu a {
  color: #999;
  border: none;
  padding: 0.6em 0 0.6em 0.6em;
}

/*
    Remove all background/borders, since we are applying them to #menu.
    */
#menu .pure-menu,
#menu .pure-menu ul {
  border: none;
  background: transparent;
}

/*
    Add that light border to separate items into groups.
    */
#menu .pure-menu ul,
#menu .pure-menu .menu-item-divided {
  border-top: 1px solid #333;
}
/*
        Change color of the anchor links on hover/focus.
        */
#menu .pure-menu li a:hover,
#menu .pure-menu li a:focus {
  background: #752a2a;
}

/*
    This styles the selected menu item `<li>`.
    */
#menu .pure-menu-selected,
#menu .pure-menu-heading {
  background: #752a2a;
}
/*
        This styles a link within a selected menu item `<li>`.
        */
#menu .pure-menu-selected a {
  color: #fff;
}

/*
    This styles the menu heading.
    */
#menu .pure-menu-heading {
  font-size: 110%;
  color: #fff;
  margin: 0;
}

.menu-link {
  position: fixed;
  display: block; /* show this only on small screens */
  top: 0;
  left: 0; /* "#menu width" */
  background: #000;
  background: rgba(0, 0, 0, 0.7);
  font-size: 10px; /* change this value to increase/decrease button size */
  z-index: 10;
  width: 2em;
  height: auto;
  padding: 2.1em 1.6em;
}

.menu-link:hover,
.menu-link:focus {
  background: #000;
}

.menu-link span {
  position: relative;
  display: block;
}

.menu-link span,
.menu-link span:before,
.menu-link span:after {
  background-color: #fff;
  pointer-events: none;
  width: 100%;
  height: 0.2em;
}

.menu-link span:before,
.menu-link span:after {
  position: absolute;
  margin-top: -0.6em;
  content: ' ';
}

.menu-link span:after {
  margin-top: 0.6em;
}

@media (min-width: 48em) {
  .header,
  .content {
    padding-left: 2em;
    padding-right: 2em;
  }

  #layout {
    padding-left: 150px;
    left: 0;
  }
  #menu {
    left: 150px;
  }

  .menu-link {
    position: fixed;
    left: 150px;
    display: none;
  }

  #layout.active .menu-link {
    left: 150px;
  }
}

@media (max-width: 48em) {
  /* Only apply this when the window is small. Otherwise, the following
    case results in extra padding on the left:
        * Make the window small.
        * Tap the menu to trigger the active state.
        * Make the window large again.
    */
  #layout.active {
    position: relative;
    left: 150px;
  }
}
</style>
