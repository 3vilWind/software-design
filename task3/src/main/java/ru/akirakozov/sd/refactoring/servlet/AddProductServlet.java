package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.gateways.Renderer;
import ru.akirakozov.sd.refactoring.model.ProductIn;
import ru.akirakozov.sd.refactoring.service.ProductService;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

/**
 * @author akirakozov
 */
public class AddProductServlet extends HttpServlet {
    protected ProductService productService;
    protected Renderer renderer;

    public AddProductServlet(ProductService productService, Renderer renderer) {
        this.productService = productService;
        this.renderer = renderer;
    }

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String name = request.getParameter("name");
        int price = Integer.parseInt(request.getParameter("price"));

        this.productService.addProduct(new ProductIn(name, price));

        response.getWriter().println(renderer.renderSuccessfulAddProductResponse());
        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }
}
