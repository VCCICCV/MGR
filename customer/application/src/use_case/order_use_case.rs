// public interface CheckoutService {

//     // 下单
//     OrderDTO checkout(@Valid CheckoutCommand cmd);

//     // 支付成功
//     OrderDTO payReceived(@Valid PaymentReceivedEvent event);

//     // 支付取消
//     OrderDTO payCanceled(@Valid PaymentCanceledEvent event);

//     // 发货
//     OrderDTO packageSent(@Valid PackageSentEvent event);

//     // 收货
//     OrderDTO delivered(@Valid DeliveredEvent event);

//     // 批量查询
//     List<OrderDTO> query(OrderQuery query);

//     // 单个查询
//     OrderDTO getOrder(Long orderId);
// }
