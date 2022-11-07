package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.gateways.Renderer;
import ru.akirakozov.sd.refactoring.service.ProductService;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

/**
 * @author akirakozov
 */
public class GetProductsServlet extends HttpServlet {
    protected ProductService productService;
    protected Renderer renderer;

    public GetProductsServlet(ProductService productService, Renderer renderer) {
        this.productService = productService;
        this.renderer = renderer;
    }

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        response.getWriter().println(renderer.renderProductsResponse(productService.getAllProducts()));
        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }
}
