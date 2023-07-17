<script>
    import {TopBar, LeftRail, MainPage} from '$lib'
  
  </script>
  
  <div class="app-container">
    <TopBar />
  
    <div class="content-container">
      <LeftRail />
  
      <div class="main-container">
        <MainPage />
      </div>
    </div>
  </div>
  
  <style>
    .app-container {
      display: flex;
      flex-direction: column;
      height: 100vh;
    }
  
    .content-container {
      display: flex;
      flex: 1;
      overflow: hidden;
    }
  
    .main-container {
      flex: 1;
      overflow-y: auto;
    }
  </style>
  
