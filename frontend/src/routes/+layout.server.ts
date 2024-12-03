export const load = ({ locals }) => ({
  user: locals.user ? ({ name: locals.user.name, picture: locals.user.picture }) : null,
});
