# Objective
The aim here is to build a tool that plots the volatility surface of a cryptocurrency in real-time using option data from Deribit. 

We will take in a stream of data from the Deribit websocket API and extract some key properties:
- Expiry date
- Strike
- Implied volatility

Fortunately, Deribit gives us this data immediately in the API call. Our job will be to organise and structure the data so that we can easily plot it in 3D. We will then need to render the surface (perhaps using the three-d crate) and ideally host it on a server. I have not worked with 3D plots outside of Python so I anticipate that this will be an interesting project.

# Usage
1. Clone the repo to use it locally with git clone `https://github.com/pbht/volatility-surface.git`
2. cd into the repo with `cd arbitrage-timer`
3. Build with `cargo build --release`
4. Run with `./target/release/volatility-surface`. You can disable either the puts or calls by using the `--puts=false` or `--calls=false` flags. Setting both to false will render empty axes

# Todo
- [x] Actual error handling - we don't want to rely on .unwrap() where it can be avoided
- [x] Websocket integration
- [x] Differentiate between puts and calls
- [x] Complete Delaunay triangulation
- [x] Render surface
- [x] Maintain persistent state, edit only necessary points 
- [x] Refactor everything from the main function. In theory, I just want this to be the render loop while we pass off data collection and processing to a separate function
- [ ] Axes titles and scales
- [ ] Smoothing
- [ ] (Long-term) integrate this render into a web application and host it for easy viewing (could be an interesting introduction to WASM)
- [ ] Fix scale so that subsequent frames are comparable
- [ ] Remove redundant points when plotting puts and calls at the same time
- [x] Improve colour scheme

# Results
![BTC Volatility Surface Render 1](resources/BTC-volatility-surface-render-1.png) \
Here we have the first render of the test data. The x axis (red) shows strike price, the y axis (green) shows time to expiry, and the z axis (blue) shows implied volatility. The render is highly fragmented and I couldn't manage to fix this despite adding anti-aliasing and using over 1,000 datapoints. The next step may be to have a smoothing function.
