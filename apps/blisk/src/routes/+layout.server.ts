export const load = ({ locals }) => ({
  user: locals.user ? ({ name: locals.user.name }) : null,
});
