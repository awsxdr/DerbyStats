import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import { HomePage } from './components/pages';
import { ColorTest } from './components/pages/ColorTest';

const router = createBrowserRouter([
    {
        path: '/',
        element: <HomePage />,
    },
    {
        path: '/tests',
        children: [
            {
                path: 'colors',
                element: <ColorTest />
            }
        ]
    }
])

export const Routes = () => (
    <RouterProvider router={router} />
);