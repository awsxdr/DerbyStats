import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import { HomePage } from './components/pages';

const router = createBrowserRouter([
    {
        path: '/',
        element: <HomePage />,
    },
])

export const Routes = () => (
    <RouterProvider router={router} />
);