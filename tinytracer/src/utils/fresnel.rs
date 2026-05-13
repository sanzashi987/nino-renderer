pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
  // Use Schlick's approximation for reflectance.
  let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
  r0 = r0 * r0;
  return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
