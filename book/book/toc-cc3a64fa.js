// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction.html">Introduction</a></span></li><li class="chapter-item expanded "><li class="part-title">Essentials</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="essentials/index.html"><strong aria-hidden="true">1.</strong> Essentials</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="essentials/understanding-the-metal-4-core-api.html"><strong aria-hidden="true">1.1.</strong> Understanding the Metal 4 core API</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="essentials/drawing-a-triangle-with-metal-4.html"><strong aria-hidden="true">1.2.</strong> Drawing a triangle with Metal 4</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="essentials/performing-calculations-on-a-gpu.html"><strong aria-hidden="true">1.3.</strong> Performing calculations on a GPU</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="essentials/using-metal-to-draw-a-view's-contents.html"><strong aria-hidden="true">1.4.</strong> Using Metal to draw a view’s contents</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Samples</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="samples/index.html"><strong aria-hidden="true">2.</strong> Samples</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="samples/metal-sample-code-library.html"><strong aria-hidden="true">2.1.</strong> Metal sample code library</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">GPU devices</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="gpu-devices/index.html"><strong aria-hidden="true">3.</strong> GPU devices</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="gpu-devices/gpu-devices-and-work-submission.html"><strong aria-hidden="true">3.1.</strong> GPU devices and work submission</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Command encoders</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/index.html"><strong aria-hidden="true">4.</strong> Command encoders</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/render-passes.html"><strong aria-hidden="true">4.1.</strong> Render passes</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/compute-passes.html"><strong aria-hidden="true">4.2.</strong> Compute passes</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/machine-learning-passes.html"><strong aria-hidden="true">4.3.</strong> Machine-learning passes</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/blit-passes.html"><strong aria-hidden="true">4.4.</strong> Blit passes</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/indirect-command-encoding.html"><strong aria-hidden="true">4.5.</strong> Indirect command encoding</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="command-encoders/ray-tracing-with-acceleration-structures.html"><strong aria-hidden="true">4.6.</strong> Ray tracing with acceleration structures</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Resources</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/index.html"><strong aria-hidden="true">5.</strong> Resources</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/resource-fundamentals.html"><strong aria-hidden="true">5.1.</strong> Resource fundamentals</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/buffers.html"><strong aria-hidden="true">5.2.</strong> Buffers</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Textures</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/index.html"><strong aria-hidden="true">6.</strong> Textures</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/memory-heaps.html"><strong aria-hidden="true">6.1.</strong> Memory heaps</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/resource-loading.html"><strong aria-hidden="true">6.2.</strong> Resource loading</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/resource-synchronization.html"><strong aria-hidden="true">6.3.</strong> Resource synchronization</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/combining-blit-and-compute-operations-in-a-single-pass.html"><strong aria-hidden="true">6.4.</strong> Combining blit and compute operations in a single pass</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/reading-pixel-data-from-a-drawable-texture.html"><strong aria-hidden="true">6.5.</strong> Reading pixel data from a drawable texture</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/creating-and-sampling-textures.html"><strong aria-hidden="true">6.6.</strong> Creating and sampling textures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="textures/streaming-large-images-with-metal-sparse-textures.html"><strong aria-hidden="true">6.7.</strong> Streaming large images with Metal sparse textures</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Shader compilation and libraries</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shader-compilation-and-libraries/index.html"><strong aria-hidden="true">7.</strong> Shader compilation and libraries</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shader-compilation-and-libraries/using-the-metal-4-compilation-api.html"><strong aria-hidden="true">7.1.</strong> Using the Metal 4 compilation API</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shader-compilation-and-libraries/shader-libraries.html"><strong aria-hidden="true">7.2.</strong> Shader libraries</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shader-compilation-and-libraries/using-function-specialization-to-build-pipeline-variants.html"><strong aria-hidden="true">7.3.</strong> Using function specialization to build pipeline variants</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Presentation</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="presentation/index.html"><strong aria-hidden="true">8.</strong> Presentation</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="presentation/managing-your-game-window-for-metal-in-macos.html"><strong aria-hidden="true">8.1.</strong> Managing your game window for Metal in macOS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="presentation/managing-your-metal-app-window-in-ipados.html"><strong aria-hidden="true">8.2.</strong> Managing your Metal app window in iPadOS</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="presentation/adapting-your-game-interface-for-smaller-screens.html"><strong aria-hidden="true">8.3.</strong> Adapting your game interface for smaller screens</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="presentation/onscreen-presentation.html"><strong aria-hidden="true">8.4.</strong> Onscreen presentation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="presentation/hdr-content.html"><strong aria-hidden="true">8.5.</strong> HDR content</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Developer tools</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/index.html"><strong aria-hidden="true">9.</strong> Developer tools</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/supporting-simulator-in-a-metal-app.html"><strong aria-hidden="true">9.1.</strong> Supporting Simulator in a Metal app</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/capturing-metal-commands-programmatically.html"><strong aria-hidden="true">9.2.</strong> Capturing Metal commands programmatically</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/logging-shader-debug-messages.html"><strong aria-hidden="true">9.3.</strong> Logging shader debug messages</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/developing-metal-apps-that-run-in-simulator.html"><strong aria-hidden="true">9.4.</strong> Developing Metal apps that run in Simulator</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/improving-your-games-graphics-performance-and-settings.html"><strong aria-hidden="true">9.5.</strong> Improving your game’s graphics performance and settings</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/gpu-counters-and-counter-sample-buffers.html"><strong aria-hidden="true">9.6.</strong> GPU counters and counter sample buffers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="developer-tools/metal-debugging-types.html"><strong aria-hidden="true">9.7.</strong> Metal debugging types</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Apple silicon</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/index.html"><strong aria-hidden="true">10.</strong> Apple silicon</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering.html"><strong aria-hidden="true">10.1.</strong> Tailor your apps for Apple GPUs and tile-based deferred rendering</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-structures.html"><strong aria-hidden="true">10.2.</strong> Metal structures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-enumerations.html"><strong aria-hidden="true">10.3.</strong> Metal enumerations</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-constants.html"><strong aria-hidden="true">10.4.</strong> Metal constants</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-functions.html"><strong aria-hidden="true">10.5.</strong> Metal functions</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-data-types.html"><strong aria-hidden="true">10.6.</strong> Metal data types</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-variables.html"><strong aria-hidden="true">10.7.</strong> Metal variables</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="apple-silicon/metal-macros.html"><strong aria-hidden="true">10.8.</strong> Metal macros</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Compute workflows</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="compute-workflows/index.html"><strong aria-hidden="true">11.</strong> Compute workflows</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="compute-workflows/performing-calculations-on-a-gpu.html"><strong aria-hidden="true">11.1.</strong> Performing calculations on a GPU</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="compute-workflows/selecting-device-objects-for-compute-processing.html"><strong aria-hidden="true">11.2.</strong> Selecting device objects for compute processing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="compute-workflows/customizing-a-tensorflow-operation.html"><strong aria-hidden="true">11.3.</strong> Customizing a TensorFlow operation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="compute-workflows/customizing-a-pytorch-operation.html"><strong aria-hidden="true">11.4.</strong> Customizing a PyTorch operation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="compute-workflows/running-a-machine-learning-model-on-the-gpu-timeline.html"><strong aria-hidden="true">11.5.</strong> Running a machine learning model on the GPU timeline</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Render workflows</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/index.html"><strong aria-hidden="true">12.</strong> Render workflows</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/using-metal-to-draw-a-view's-contents.html"><strong aria-hidden="true">12.1.</strong> Using Metal to draw a view’s contents</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/drawing-a-triangle-with-metal-4.html"><strong aria-hidden="true">12.2.</strong> Drawing a triangle with Metal 4</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/selecting-device-objects-for-graphics-rendering.html"><strong aria-hidden="true">12.3.</strong> Selecting device objects for graphics rendering</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/customizing-render-pass-setup.html"><strong aria-hidden="true">12.4.</strong> Customizing render pass setup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/creating-a-custom-metal-view.html"><strong aria-hidden="true">12.5.</strong> Creating a custom Metal view</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/calculating-primitive-visibility-using-depth-testing.html"><strong aria-hidden="true">12.6.</strong> Calculating primitive visibility using depth testing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/encoding-indirect-command-buffers-on-the-cpu.html"><strong aria-hidden="true">12.7.</strong> Encoding indirect command buffers on the CPU</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/implementing-order-independent-transparency-with-image-blocks.html"><strong aria-hidden="true">12.8.</strong> Implementing order-independent transparency with image blocks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/loading-textures-and-models-using-metal-fast-resource-loading.html"><strong aria-hidden="true">12.9.</strong> Loading textures and models using Metal fast resource loading</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/adjusting-the-level-of-detail-using-metal-mesh-shaders.html"><strong aria-hidden="true">12.10.</strong> Adjusting the level of detail using Metal mesh shaders</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/creating-a-3d-application-with-hydra-rendering.html"><strong aria-hidden="true">12.11.</strong> Creating a 3D application with hydra rendering</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/culling-occluded-geometry-using-the-visibility-result-buffer.html"><strong aria-hidden="true">12.12.</strong> Culling occluded geometry using the visibility result buffer</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/improving-edge-rendering-quality-with-multisample-antialiasing-msaa.html"><strong aria-hidden="true">12.13.</strong> Improving edge-rendering quality with multisample antialiasing (MSAA)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="render-workflows/achieving-smooth-frame-rates-with-a-metal-display-link.html"><strong aria-hidden="true">12.14.</strong> Achieving smooth frame rates with a Metal display link</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Argument buffers</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="argument-buffers/index.html"><strong aria-hidden="true">13.</strong> Argument buffers</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="argument-buffers/managing-groups-of-resources-with-argument-buffers.html"><strong aria-hidden="true">13.1.</strong> Managing groups of resources with argument buffers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="argument-buffers/using-argument-buffers-with-resource-heaps.html"><strong aria-hidden="true">13.2.</strong> Using argument buffers with resource heaps</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="argument-buffers/encoding-argument-buffers-on-the-gpu.html"><strong aria-hidden="true">13.3.</strong> Encoding argument buffers on the GPU</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="argument-buffers/rendering-terrain-dynamically-with-argument-buffers.html"><strong aria-hidden="true">13.4.</strong> Rendering terrain dynamically with argument buffers</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Shaders</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shaders/index.html"><strong aria-hidden="true">14.</strong> Shaders</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shaders/creating-a-metal-dynamic-library.html"><strong aria-hidden="true">14.1.</strong> Creating a Metal dynamic library</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shaders/using-function-specialization-to-build-pipeline-variants.html"><strong aria-hidden="true">14.2.</strong> Using function specialization to build pipeline variants</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Synchronization</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="synchronization/index.html"><strong aria-hidden="true">15.</strong> Synchronization</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="synchronization/synchronizing-cpu-and-gpu-work.html"><strong aria-hidden="true">15.1.</strong> Synchronizing CPU and GPU work</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="synchronization/implementing-a-multistage-image-filter-using-heaps-and-events.html"><strong aria-hidden="true">15.2.</strong> Implementing a multistage image filter using heaps and events</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="synchronization/implementing-a-multistage-image-filter-using-heaps-and-fences.html"><strong aria-hidden="true">15.3.</strong> Implementing a multistage image filter using heaps and fences</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Lighting techniques</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="lighting-techniques/index.html"><strong aria-hidden="true">16.</strong> Lighting techniques</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="lighting-techniques/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders.html"><strong aria-hidden="true">16.1.</strong> Rendering a scene with forward plus lighting using tile shaders</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="lighting-techniques/rendering-a-scene-with-deferred-lighting-in-objective-c.html"><strong aria-hidden="true">16.2.</strong> Rendering a scene with deferred lighting in Objective-C</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="lighting-techniques/rendering-a-scene-with-deferred-lighting-in-swift.html"><strong aria-hidden="true">16.3.</strong> Rendering a scene with deferred lighting in Swift</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="lighting-techniques/rendering-a-scene-with-deferred-lighting-in-c++.html"><strong aria-hidden="true">16.4.</strong> Rendering a scene with deferred lighting in C++</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="lighting-techniques/rendering-reflections-with-fewer-render-passes.html"><strong aria-hidden="true">16.5.</strong> Rendering reflections with fewer render passes</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Multiple techniques</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="multiple-techniques/index.html"><strong aria-hidden="true">17.</strong> Multiple techniques</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="multiple-techniques/modern-rendering-with-metal.html"><strong aria-hidden="true">17.1.</strong> Modern rendering with Metal</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="multiple-techniques/encoding-indirect-command-buffers-on-the-gpu.html"><strong aria-hidden="true">17.2.</strong> Encoding indirect command buffers on the GPU</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">Ray tracing</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ray-tracing/index.html"><strong aria-hidden="true">18.</strong> Ray tracing</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ray-tracing/rendering-reflections-in-real-time-using-ray-tracing.html"><strong aria-hidden="true">18.1.</strong> Rendering reflections in real time using ray tracing</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ray-tracing/accelerating-ray-tracing-using-metal.html"><strong aria-hidden="true">18.2.</strong> Accelerating ray tracing using Metal</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ray-tracing/control-the-ray-tracing-process-using-intersection-queries.html"><strong aria-hidden="true">18.3.</strong> Control the ray tracing process using intersection queries</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ray-tracing/accelerating-ray-tracing-and-motion-blur-using-metal.html"><strong aria-hidden="true">18.4.</strong> Accelerating ray tracing and motion blur using Metal</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ray-tracing/rendering-a-curve-primitive-in-a-ray-tracing-scene.html"><strong aria-hidden="true">18.5.</strong> Rendering a curve primitive in a ray tracing scene</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">HDR</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="hdr/index.html"><strong aria-hidden="true">19.</strong> HDR</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="hdr/processing-hdr-images-with-metal.html"><strong aria-hidden="true">19.1.</strong> Processing HDR images with Metal</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">OpenGL</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="opengl/index.html"><strong aria-hidden="true">20.</strong> OpenGL</a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="opengl/migrating-opengl-code-to-metal.html"><strong aria-hidden="true">20.1.</strong> Migrating OpenGL code to Metal</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="opengl/mixing-metal-and-opengl-rendering-in-a-view.html"><strong aria-hidden="true">20.2.</strong> Mixing Metal and OpenGL rendering in a view</a></span></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

