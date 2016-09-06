/* automatically generated by rust-bindgen */
#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

/// Determines how should tessTesselate interpret it's input, see [OpenGL tesselation docs](http://www.glprogramming.com/red/chapter11.html) for more
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum TessWindingRule {
    TESS_WINDING_ODD = 0,
    TESS_WINDING_NONZERO = 1,
    TESS_WINDING_POSITIVE = 2,
    TESS_WINDING_NEGATIVE = 3,
    TESS_WINDING_ABS_GEQ_TWO = 4,
}
/// defines, what action tessTesselate is performing, tesselation, tesselation with neighboring polygons, or creating polygon contours
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum TessElementType {
    TESS_POLYGONS = 0,
    TESS_CONNECTED_POLYGONS = 1,
    TESS_BOUNDARY_CONTOURS = 2,
}
/// typedef's from C
pub type TESSreal = f32;
pub type TESSindex = ::std::os::raw::c_int;
/// Used only as opaque pointer to c structure
pub enum TESStesselator { }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct TESSalloc {
    pub memalloc: unsafe extern "C" fn(userData:  *mut ::std::os::raw::c_void,
                                       size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_void,
    pub memrealloc: unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void,
                                         ptr: *mut ::std::os::raw::c_void,
                                         size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_void,
    pub memfree: unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void,
                                      ptr: *mut ::std::os::raw::c_void),
    pub userData: *mut ::std::os::raw::c_void,
    pub meshEdgeBucketSize: ::std::os::raw::c_int,
    pub meshVertexBucketSize: ::std::os::raw::c_int,
    pub meshFaceBucketSize: ::std::os::raw::c_int,
    pub dictNodeBucketSize: ::std::os::raw::c_int,
    pub regionBucketSize: ::std::os::raw::c_int,
    pub extraVertices: ::std::os::raw::c_int,
}
impl ::std::default::Default for TESSalloc {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "tess2", kind = "dylib")]
extern "C" {
    /// Creates new Tesselator object with specified or default allocator or uses default.
    ///
    /// # Arguments
    /// * `alloc` - Tesselator allocation configuration, if null, then default is used
    pub fn tessNewTess(alloc: *mut TESSalloc) -> *mut TESStesselator;
    /// Destroys tesselator and all resources associated
    pub fn tessDeleteTess(tess: *mut TESStesselator);
    /// Adds new contour to tesselator, this function copies vertices into internal buffer
    ///
    /// # Arguments
    /// * `size`    - specifies number of floats per vertex
    /// * `pointer` - pointer to data
    /// * `stride`  - stride between vertices ( in bytes )
    /// * `count`   - number of vertices to add
    pub fn tessAddContour(tess: *mut TESStesselator,
                          size: ::std::os::raw::c_int,
                          pointer: *const ::std::os::raw::c_void,
                          stride: ::std::os::raw::c_int,
                          count: ::std::os::raw::c_int);
    /// Tesselates stored paths
    ///
    /// # Arguments
    /// * `windingRule` - see [OpenGL tesselation docs](http://www.glprogramming.com/red/chapter11.html) for more
    /// * `elementType` - specifies output type
    /// * `polySize`    - maximum number of vertices per generated polygon
    /// * `vertexSize`  - numer of coordinates per vertex, can be 2 or 3
    /// * `normal`      - the normal of the input contours, if null the normal is calculated automatically.
    ///
    /// First call to this function invalidates internal input buffers, so every call to tessAddContour after calling tessTesselate
    /// is not affected by previous data
    pub fn tessTesselate(tess: *mut TESStesselator,
                         windingRule: TessWindingRule,
                         elementType: TessElementType,
                         polySize: ::std::os::raw::c_int,
                         vertexSize: ::std::os::raw::c_int,
                         normal: *const TESSreal) -> ::std::os::raw::c_int;
    /// Returns number of vertices in internal output buffer
    pub fn tessGetVertexCount(tess: *mut TESStesselator) -> ::std::os::raw::c_int;
    /// Returns pointer to internal vertex output buffer
    pub fn tessGetVertices(tess: *mut TESStesselator) -> *const TESSreal;
    /// Returns pointer to vertex index buffer
    ///
    /// Vertex index buffer is used to map output vertices onto input vertices.
    ///
    /// internally generated vertices are assigned value of -1
    pub fn tessGetVertexIndices(tess: *mut TESStesselator) -> *const TESSindex;
    /// Returns number of generated elements
    ///
    /// Meaning of this number depends on elementType specified when calling tessTesselate
    pub fn tessGetElementCount(tess: *mut TESStesselator) -> ::std::os::raw::c_int;
    /// Returns element buffer
    ///
    /// # Warning!
    /// Size of this buffer is not tessGetElementCount ints.
    ///
    /// Size of this buffer depends on values provided when calling tessTesselate
    ///
    /// if element type is TESS_POLYGONS, then this array contains tessGetElementCount * polySize integers.
    /// array can be divided into tessGetElementCount slices of polySize length.
    /// Each slice contains indices to vertices in tessGetVertices that create this polygon.
    /// If polygon has less vertices than polySize, remaining indices are -1
    ///
    /// if element type is TESS_CONNECTED_POLYGONS, this array contains tessGetElementCount * polysize
    ///
    /// if element type is TESS_BOUNDARY_CONTOURS, this array contains tessGetElementCount * 2 integers
    /// each pair of values determines [position, length] of polygon contours stored in vertices array
    pub fn tessGetElements(tess: *mut TESStesselator) -> *const TESSindex;
}

#[test]
fn test_link(){
    unsafe {
        let a = tessNewTess(0 as *mut _);
        tessDeleteTess(a);
    }
}

#[test]
fn test_basic_tess(){
    let mut data = [
        0.0,0.0,
       // 0.3,0.3f32,
        1.0,0.0f32,
        1.0,1.0,
        0.0,1.0,
    ];unsafe {
        let nvp = 3usize;
        let fpv = 2;
        let bpv = 2 * std::mem::size_of::<f32>();
        let mut tess = tessNewTess(0 as *mut _);
        tessAddContour(tess,2,data.as_ptr() as _, 2 * std::mem::size_of::<f32>() as i32,(data.len()/2) as _);

       /* tessAddContour(tess,2,data.as_ptr() as _, 2 * std::mem::size_of::<f32>() as i32,(data.len()/2) as _); data[0] = -1.0;
        data[1] = -1.0;

        tessAddContour(tess,2,data.as_ptr() as _, 2 * std::mem::size_of::<f32>() as i32,(data.len()/2) as _);
        */
      //  tessAddContour(tess,2,data.as_ptr() as _,0,(data.len()/2) as _);

        println!("STATUS:{:?}", tessTesselate(tess,TessWindingRule::TESS_WINDING_POSITIVE,TessElementType::TESS_POLYGONS,nvp as i32,2,0 as *const _));
        println!("Elems:{:?} {:?}",tessGetElements(tess),tessGetElementCount(tess));
        println!("Elems:{:?} {:?}",tessGetVertices(tess),tessGetVertexCount(tess));
        println!("Elems:{:?} {:?}",tessGetVertexIndices(tess),tessGetVertexCount(tess));
        println!("Elems:{:?}",std::slice::from_raw_parts(tessGetElements(tess),((tessGetElementCount(tess) as usize)*nvp)));
        println!("Verts:{:?}",std::slice::from_raw_parts(tessGetVertices(tess) as *const f32,tessGetVertexCount(tess) as usize * fpv));
        println!("Indis:{:?}",std::slice::from_raw_parts(tessGetVertexIndices(tess),tessGetVertexCount(tess) as _));


    }
}